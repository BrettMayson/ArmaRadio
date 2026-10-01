// Based on the original Live Radio implementation by BrettMayson.
// Playback and compatibility changes by Joncantplay.
use std::{
    collections::HashMap,
    sync::{
        atomic::AtomicU8,
        mpsc::{self, Receiver, Sender},
        Mutex, OnceLock, RwLock,
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

#[derive(Debug)]
pub struct SoundSource {
    channel: Sender<SoundCommand>,
}

impl SoundSource {
    pub fn new(ctx: Context, id: String, url: String, gain: f32) -> Self {
        let (tx, rx): (Sender<SoundCommand>, Receiver<SoundCommand>) = mpsc::channel();
        std::thread::spawn(move || {
            debug!("Starting source `{}`", id);
            let stream = Streams::listen(url);
            let Some(listener) = Listener::get() else {
                return;
            };
            let Ok(mut source) = listener.new_streaming_source() else {
                error!("Error creating source");
                return;
            };
            if let Err(error) =
                source.set_soft_spatialization(alto::SoftSourceSpatialization::Enabled)
            {
                error!("Error setting soft spatialization for {id}: {error}");
                return;
            }
            if let Err(error) = source.set_gain(
                gain * ctx
                    .group()
                    .get::<AtomicU8>()
                    .map(|gain| gain.load(std::sync::atomic::Ordering::Relaxed))
                    .unwrap_or(255) as f32
                    / 255.0,
            ) {
                error!("Error setting initial gain for {id}: {error}");
                return;
            }
            let mut specific_gain = gain;
            'outer: loop {
                while let Ok(command) = rx.try_recv() {
                    match command {
                        SoundCommand::SetPos(pos) => {
                            if source.set_position(pos).is_err() {
                                error!("Error setting position for {}", id);
                            }
                        }
                        SoundCommand::SetGain(gain) => {
                            debug!("Setting gain to {} for {}", gain, id);
                            specific_gain = gain;
                            if source
                                .set_gain(
                                    gain * ctx
                                        .group()
                                        .get::<AtomicU8>()
                                        .map(|gain| gain.load(std::sync::atomic::Ordering::Relaxed))
                                        .unwrap_or(255)
                                        as f32
                                        / 255.0,
                                )
                                .is_err()
                            {
                                error!("Error setting gain");
                            }
                        }
                        SoundCommand::RefreshGain => {
                            debug!("Refreshing gain for {}", id);
                            if source
                                .set_gain(
                                    specific_gain
                                        * ctx
                                            .group()
                                            .get::<AtomicU8>()
                                            .map(|gain| {
                                                gain.load(std::sync::atomic::Ordering::Relaxed)
                                            })
                                            .unwrap_or(255)
                                            as f32
                                        / 255.0,
                                )
                                .is_err()
                            {
                                error!("Error setting gain");
                            }
                        }
                        SoundCommand::Destroy => {
                            debug!("Source `{}` has been told to destroy", id);
                            source.stop();
                            break 'outer;
                        }
                    }
                }
                match stream.receiver.try_recv() {
                    Ok(recv) => {
                        match recv {
                            StreamPacket::Data(samples, freq) => {
                                let buffer = if source.buffers_processed() > 200 {
                                    if let Ok(mut buffer) = source.unqueue_buffer() {
                                        if let Err(e) = buffer.set_data(samples, freq) {
                                            error!(
                                                "Error setting buffer sample data for {}: {}",
                                                id, e
                                            );
                                            continue;
                                        }
                                        buffer
                                    } else {
                                        let Some(listener) = Listener::get() else {
                                            return;
                                        };
                                        let Ok(buffer) = listener.new_buffer(samples, freq) else {
                                            error!("Error creating buffer for {}", id);
                                            continue;
                                        };
                                        buffer
                                    }
                                } else {
                                    let Some(listener) = Listener::get() else {
                                        return;
                                    };
                                    let Ok(buffer) = listener.new_buffer(samples, freq) else {
                                        error!("Error creating buffer for {}", id);
                                        continue;
                                    };
                                    buffer
                                };
                                if let Err(e) = source.queue_buffer(buffer) {
                                    error!(
                                        "killing thread, error queueing buffer for {}: {}",
                                        id, e
                                    );
                                    return;
                                }
                                if source.state() != alto::SourceState::Playing
                                    && source.buffers_queued() > 75
                                {
                                    info!("Playing source for {}, {:?}", id, source.state());
                                    source.play();
                                }
                            }
                            StreamPacket::Title(title) => {
                                if ctx
                                    .callback_data(
                                        "live_radio",
                                        "title",
                                        Some(vec![id.to_string(), title]),
                                    )
                                    .is_err()
                                {
                                    // arma is probably closed
                                    break;
                                }
                            }
                            StreamPacket::AlbumArt(path) => {
                                if ctx.callback_data("live_radio", "album_art", Some(vec![id.to_string(), path])).is_err() {
                                    break;
                                }
                            }
                            StreamPacket::Error(message) => {
                                let _ = ctx.callback_data(
                                    "live_radio", "error", Some(vec![id.clone(), message]),
                                );
                            }
                            StreamPacket::Close => {
                                debug!("Stream closed for {}", id);
                                source.stop();
                                break;
                            }
                            StreamPacket::Check => {
                                // noop
                            }
                        }
                    }
                    Err(TryRecvError::Empty) => {
                        std::thread::sleep(std::time::Duration::from_millis(16));
                    }
                    Err(TryRecvError::Disconnected) => {
                        error!("Stream receiver disconnected for {}", id);
                        break;
                    }
                }
            }
            debug!("Source `{}` has died", id);
        });
        Self { channel: tx }
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
