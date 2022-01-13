use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver, Sender, TryRecvError},
        Arc, Mutex, RwLock,
    },
    time::SystemTime,
};

use alto::Source;
use arma_rs::{Context, Group};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::blocking::Client;
use simplemad::Decoder;

use crate::{listener::LISTENER, station::Station, vector3::Vector3};

lazy_static::lazy_static! {
    static ref SOURCES: RwLock<HashMap<String, Mutex<SoundSource>>> = RwLock::new(HashMap::new());
}

#[derive(Debug)]
pub struct SoundSource {
    position: Vector3,
    velocity: Vector3,
    gain: f32,
    time: SystemTime,
    channel: Sender<[f32; 7]>,
    pub station: String,
}

impl SoundSource {
    pub fn new(ctx: Context, id: String, station: String, gain: f32) -> Self {
        let (tx, rx): (Sender<[f32; 7]>, Receiver<[f32; 7]>) = mpsc::channel();
        let s = station.clone();
        std::thread::spawn(move || {
            let client = Client::new();
            info!("Starting Radio. URL: {}", station);
            let mut request = client.get(&station);
            request = request.header("Icy-MetaData", "1");
            let decoder = Decoder::decode(Station::new(ctx, request.send().unwrap(), id)).unwrap();
            let stream = Arc::new(Mutex::new(LISTENER.new_streaming_source().unwrap()));
            if stream
                .lock()
                .unwrap()
                .set_soft_spatialization(alto::SoftSourceSpatialization::Enabled)
                .is_err()
            {
                warn!("Error setting soft spatialization");
            }
            if stream.lock().unwrap().set_max_gain(2_f32).is_err() {
                warn!("Error setting max gain");
            };
            if stream.lock().unwrap().set_gain(gain).is_err() {
                warn!("Error setting gain");
            };
            // stream.lock().unwrap().set_rolloff_factor(1.0);
            let (txi, rxi): (Sender<()>, Receiver<()>) = mpsc::channel();
            let inner_stream = stream.clone();
            std::thread::spawn(move || loop {
                match rx.try_recv() {
                    Ok(message) => {
                        inner_stream
                            .lock()
                            .unwrap()
                            .set_position([message[0], message[1], message[2]])
                            .unwrap();
                        inner_stream
                            .lock()
                            .unwrap()
                            .set_velocity([message[3], message[4], message[5]])
                            .unwrap();
                        inner_stream.lock().unwrap().set_gain(message[6]).unwrap();
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Disconnected) => {
                        if txi.send(()).is_err() {
                            warn!("Error sending txi empty");
                        }
                        break;
                    }
                }
            });
            for decoding_result in decoder {
                if Err(TryRecvError::Disconnected) == rxi.try_recv() {
                    info!("Dying");
                    break;
                }
                match decoding_result {
                    Err(_) => {} // error!("Error: {:?}", e),
                    Ok(frame) => {
                        let mut samples: Vec<alto::Mono<f32>> = Vec::new();
                        for i in 0..frame.samples[0].len() {
                            samples.push(alto::Mono {
                                center: (frame.samples[0][i].to_f32()
                                    + frame.samples[1][i].to_f32())
                                    / 2.0_f32,
                            });
                        }
                        let buffer = if let Ok(mut buffer) = stream.lock().unwrap().unqueue_buffer()
                        {
                            if buffer.set_data(samples, frame.sample_rate as i32).is_err() {
                                warn!("Error setting buffer sample data");
                            }
                            buffer
                        } else {
                            LISTENER
                                .new_buffer(samples, frame.sample_rate as i32)
                                .unwrap()
                        };
                        stream.lock().unwrap().queue_buffer(buffer).unwrap();
                        if stream.lock().unwrap().state() != alto::SourceState::Playing {
                            stream.lock().unwrap().play();
                        }
                    }
                }
            }
        });
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            gain,
            time: SystemTime::now(),
            channel: tx,
            station: s,
        }
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        let old = self.time;
        self.time = SystemTime::now();
        let dif = self.time.duration_since(old).unwrap();
        let elapsed: f32 = (dif.as_secs() as f32) + (dif.subsec_nanos() as f32 / 1_000_000_000.0);

        self.velocity = Vector3::new(
            (position[0] - self.position.x) / elapsed,
            (position[1] - self.position.y) / elapsed,
            (position[2] - self.position.z) / elapsed,
        );
        self.position.x = position[0];
        self.position.y = position[1];
        self.position.z = position[2];
        if self
            .channel
            .send([
                self.position.x,
                self.position.y,
                self.position.z,
                self.velocity.x,
                self.velocity.y,
                self.velocity.z,
                self.gain,
            ])
            .is_err()
        {
            warn!("Error sending position update");
        }
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
        if self
            .channel
            .send([
                self.position.x,
                self.position.y,
                self.position.z,
                self.velocity.x,
                self.velocity.y,
                self.velocity.z,
                self.gain,
            ])
            .is_err()
        {
            warn!("error sending gain update");
        }
    }
}

pub fn group() -> Group {
    Group::new()
        .command("new", new)
        .command("destroy", destroy)
        .command("pos", set_position)
        .command("gain", set_gain)
}

fn new(ctx: Context, source: String, gain: f32) -> String {
    let id = String::from_utf8(
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .collect::<Vec<u8>>(),
    )
    .unwrap()
    .to_lowercase();
    SOURCES.write().unwrap().insert(
        id.clone(),
        Mutex::new(SoundSource::new(ctx, id.clone(), source, gain)),
    );
    id
}

fn destroy(id: String) -> bool {
    info!("`{}` has been told to die", id);
    SOURCES
        .write()
        .unwrap()
        .remove(&id)
        .map_or(false, |source| {
            info!("`{}` was playing `{}`", id, source.lock().unwrap().station);
            true
        })
}

pub fn set_position(id: String, x: f32, y: f32, z: f32) {
    if let Some(src) = SOURCES.read().unwrap().get(&id) {
        src.lock().unwrap().set_position([x, y, z]);
    }
}

pub fn set_gain(id: String, gain: f32) {
    if let Some(src) = SOURCES.read().unwrap().get(&id) {
        src.lock().unwrap().set_gain(gain);
    }
}
