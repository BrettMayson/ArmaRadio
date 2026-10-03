// Based on the original Live Radio implementation by BrettMayson.
// Playback and compatibility changes by Joncantplay.
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex, OnceLock, RwLock,
        atomic::{AtomicBool, AtomicU8, Ordering},
        mpsc::{self, Receiver, Sender},
    },
};

use alto::Source;
use arma_rs::{Context, ContextState, Group};
use crossbeam_channel::TryRecvError;

use crate::{
    listener::Listener,
    streams::{StreamPacket, Streams},
};

pub struct Sources();

type SourceMap = RwLock<HashMap<String, Mutex<SoundSource>>>;

impl Sources {
    pub fn get() -> &'static SourceMap {
        static SOURCES: OnceLock<SourceMap> = OnceLock::new();
        SOURCES.get_or_init(|| RwLock::new(HashMap::new()))
    }
}

enum SoundCommand {
    SetPos([f32; 3]),
    SetGain(f32),
    RefreshGain,
    Destroy,
}

pub struct SoundSource {
    channel: Sender<SoundCommand>,
    cancelled: Arc<AtomicBool>,
    context: Arc<Context>,
    id: String,
}

impl SoundSource {
    pub fn new(ctx: Context, id: String, url: String, gain: f32) -> Self {
        let (tx, rx): (Sender<SoundCommand>, Receiver<SoundCommand>) = mpsc::channel();
        let cancelled = Arc::new(AtomicBool::new(false));
        let worker_cancelled = cancelled.clone();
        let ctx = Arc::new(ctx);
        let stored_context = ctx.clone();
        let stored_id = id.clone();
        std::thread::spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                Self::run(&ctx, &id, url, gain, rx, &worker_cancelled)
            }));
            if !worker_cancelled.load(Ordering::SeqCst) {
                let error = match result {
                    Ok(Ok(())) => None,
                    Ok(Err(error)) => Some(error),
                    Err(_) => Some("decode: Audio worker panicked; source stopped".to_string()),
                };
                if let Some(error) = error {
                    debug!("Source {id}: {error}");
                    let _ = ctx.callback_data("live_radio", "error", Some(vec![id.clone(), error]));
                }
                let _ =
                    ctx.callback_data("live_radio", "state", Some(vec![id, "ended".to_string()]));
            }
        });
        Self {
            channel: tx,
            cancelled,
            context: stored_context,
            id: stored_id,
        }
    }

    fn run(
        ctx: &Context,
        id: &str,
        url: String,
        gain: f32,
        rx: Receiver<SoundCommand>,
        cancelled: &AtomicBool,
    ) -> Result<(), String> {
        use std::time::{Duration, Instant};
        let listener = Listener::get().ok_or("audio: Audio device unavailable")?;
        let mut source = listener
            .new_streaming_source()
            .map_err(|e| format!("audio: Could not create source: {e}"))?;
        // Spatialization is optional on devices without AL_SOFT_source_spatialize.
        if let Err(error) = source.set_soft_spatialization(alto::SoftSourceSpatialization::Enabled)
        {
            debug!("Optional spatialization unavailable for {id}: {error}");
        }
        let global_gain = || {
            ctx.group()
                .get::<AtomicU8>()
                .map_or(255, |gain| gain.load(Ordering::Relaxed)) as f32
                / 255.0
        };
        source
            .set_gain(gain * global_gain())
            .map_err(|e| format!("audio: Gain: {e}"))?;
        if cancelled.load(Ordering::SeqCst) {
            return Ok(());
        }
        let stream = Streams::listen(url);
        let mut specific_gain = gain;
        let mut last_audio = Instant::now();
        let mut started = false;
        let mut queued_seconds = 0.0_f64;
        loop {
            if cancelled.load(Ordering::SeqCst) {
                source.stop();
                return Ok(());
            }
            while let Ok(command) = rx.try_recv() {
                match command {
                    SoundCommand::Destroy => {
                        source.stop();
                        return Ok(());
                    }
                    SoundCommand::SetPos(pos) => source
                        .set_position(pos)
                        .map_err(|e| format!("audio: Position: {e}"))?,
                    SoundCommand::SetGain(gain) => {
                        specific_gain = gain;
                        source
                            .set_gain(gain * global_gain())
                            .map_err(|e| format!("audio: Gain: {e}"))?;
                    }
                    SoundCommand::RefreshGain => source
                        .set_gain(specific_gain * global_gain())
                        .map_err(|e| format!("audio: Gain: {e}"))?,
                }
            }
            // Bound startup and a stalled live connection even if the network
            // worker is blocked waiting for HTTP/body data.
            if last_audio.elapsed() >= Duration::from_secs(10) {
                source.stop();
                return Err(if started {
                    "timeout: No audio received for 10 seconds"
                } else {
                    "timeout: Initial buffering exceeded 10 seconds"
                }
                .to_string());
            }
            while source.buffers_processed() > 0 {
                source
                    .unqueue_buffer()
                    .map_err(|e| format!("audio: Buffer cleanup: {e}"))?;
            }
            match stream.receiver.try_recv() {
                Ok(StreamPacket::Data(samples, rate)) => {
                    if samples.is_empty() || rate <= 0 {
                        continue;
                    }
                    if samples.iter().any(|sample| !sample.center.is_finite()) {
                        return Err("decode: Non-finite PCM samples".to_string());
                    }
                    let duration = samples.len() as f64 / f64::from(rate);
                    let buffer = listener
                        .new_buffer(samples, rate)
                        .map_err(|e| format!("audio: Buffer creation: {e}"))?;
                    if cancelled.load(Ordering::SeqCst) {
                        source.stop();
                        return Ok(());
                    }
                    source
                        .queue_buffer(buffer)
                        .map_err(|e| format!("audio: Buffer queue: {e}"))?;
                    // A short duration-based prebuffer works for MP3 and AAC,
                    // irrespective of how many PCM packets each decoder emits.
                    queued_seconds += duration;
                    if source.state() != alto::SourceState::Playing && queued_seconds >= 0.15 {
                        source.play();
                        if source.state() != alto::SourceState::Playing {
                            return Err("audio: Playback could not start".to_string());
                        }
                        if !started {
                            started = true;
                            debug!("Stream started successfully: {id}");
                            let _ = ctx.callback_data(
                                "live_radio",
                                "state",
                                Some(vec![id.to_string(), "started".to_string()]),
                            );
                        }
                    }
                    // Before playback actually begins, retain the absolute
                    // initial deadline; one tiny PCM packet cannot reset it.
                    if started {
                        last_audio = Instant::now();
                    }
                }
                Ok(StreamPacket::Title(title)) => {
                    let _ =
                        ctx.callback_data("live_radio", "title", Some(vec![id.to_string(), title]));
                }
                Ok(StreamPacket::AlbumArt(path)) => {
                    let _ = ctx.callback_data(
                        "live_radio",
                        "album_art",
                        Some(vec![id.to_string(), path]),
                    );
                }
                Ok(StreamPacket::Error(message)) => {
                    source.stop();
                    return Err(message);
                }
                Ok(StreamPacket::Close) | Err(TryRecvError::Disconnected) => {
                    source.stop();
                    return Err("read: Stream connection closed".to_string());
                }
                Ok(StreamPacket::Check) => {}
                Err(TryRecvError::Empty) => std::thread::sleep(Duration::from_millis(10)),
            }
        }
    }

    pub fn set_position(&self, position: [f32; 3]) {
        if self.channel.send(SoundCommand::SetPos(position)).is_err() {
            error!("error sending position update");
        }
    }

    pub fn set_gain(&self, gain: f32) {
        if self.channel.send(SoundCommand::SetGain(gain)).is_err() {
            error!("error sending gain update");
        }
    }

    pub fn refresh_gain(&self) {
        if self.channel.send(SoundCommand::RefreshGain).is_err() {
            error!("error sending gain refresh");
        }
    }
}

impl Drop for SoundSource {
    fn drop(&mut self) {
        self.cancelled.store(true, Ordering::SeqCst);
        let _ = self.context.callback_data(
            "live_radio",
            "state",
            Some(vec![self.id.clone(), "ended".to_string()]),
        );
        debug!("Dropping source");
        if self.channel.send(SoundCommand::Destroy).is_err() {
            error!("error sending destroy command");
        }
    }
}

pub fn cleanup() {
    debug!("cleaning up sources");
    match Sources::get().write() {
        Ok(mut sources) => sources.clear(),
        Err(_) => error!("Source map lock was poisoned during cleanup"),
    }
}

pub fn group() -> Group {
    let global_gain = AtomicU8::new(255);
    Group::new()
        .command("new", command_new)
        .command("destroy", command_destroy)
        .command("pos", command_set_position)
        .command("gain", command_set_gain)
        .command("global_gain", command_set_global_gain)
        .state(global_gain)
}

fn command_new(ctx: Context, id: String, source: String, gain: f32) -> String {
    match Sources::get().write() {
        Ok(mut sources) => {
            sources.insert(
                id.clone(),
                Mutex::new(SoundSource::new(ctx, id.clone(), source, gain)),
            );
            id
        }
        Err(_) => {
            error!("Source map lock was poisoned while creating a source");
            String::new()
        }
    }
}

fn command_destroy(id: String) -> bool {
    match Sources::get().write() {
        Ok(mut sources) => sources.remove(&id).is_some(),
        Err(_) => {
            error!("Source map lock was poisoned while destroying a source");
            false
        }
    }
}

pub fn command_set_position(id: String, x: f32, y: f32, z: f32) {
    let Ok(sources) = Sources::get().read() else {
        error!("Source map lock was poisoned while setting position");
        return;
    };
    if let Some(src) = sources.get(&id) {
        match src.lock() {
            Ok(src) => src.set_position([x, y, z]),
            Err(_) => error!("Source lock was poisoned while setting position for {id}"),
        }
    }
}

pub fn command_set_gain(id: String, gain: f32) {
    let Ok(sources) = Sources::get().read() else {
        error!("Source map lock was poisoned while setting gain");
        return;
    };
    if let Some(src) = sources.get(&id) {
        match src.lock() {
            Ok(src) => src.set_gain(gain),
            Err(_) => error!("Source lock was poisoned while setting gain for {id}"),
        }
    }
}

pub fn command_set_global_gain(ctx: Context, gain: f32) {
    let gain = (gain * 255.0) as u8;
    debug!("Setting global gain to {}", gain);
    if let Some(state) = ctx.group().get::<AtomicU8>() {
        state.store(gain, std::sync::atomic::Ordering::Relaxed);
    }
    let Ok(sources) = Sources::get().read() else {
        error!("Source map lock was poisoned while refreshing gain");
        return;
    };
    for (id, src) in sources.iter() {
        match src.lock() {
            Ok(src) => src.refresh_gain(),
            Err(_) => error!("Source lock was poisoned while refreshing gain for {id}"),
        }
    }
}
