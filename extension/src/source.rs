use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::SystemTime;
use std::io::{self, Read};

use alto::Source;
use reqwest::blocking::Client;

// mpeg
use simplemad::Decoder;

struct OnlineRadio<R> {
    request: R
}
impl<R: Read> Read for OnlineRadio<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.request.read(buf).map(|n| {
            n
        })
    }
}

pub struct SoundSource {
    position: Vector3,
    velocity: Vector3,
    time: SystemTime,
    channel: Sender<[f32; 6]>,
}
impl SoundSource {
    pub fn new(station: String) -> SoundSource {
        let (tx, rx): (Sender<[f32; 6]>, Receiver<[f32; 6]>) = mpsc::channel();
        std::thread::spawn(move || {
            let client = Client::new();
            println!("Station: {}", station);
            let request = client.get(&station);
            let decoder = Decoder::decode(OnlineRadio {
                request: request.send().unwrap()
            }).unwrap();
            let stream = Arc::new(Mutex::new(crate::CONTEXT.new_streaming_source().unwrap()));
            stream.lock().unwrap().play();
            let (txi, rxi): (Sender<()>, Receiver<()>) = mpsc::channel();
            let inner_stream = stream.clone();
            thread::spawn(move || {
                loop {
                    match rx.try_recv() {
                        Ok(message) => {
                            inner_stream.lock().unwrap()
                                .set_position([message[0], message[1], message[2]])
                                .unwrap();
                            inner_stream.lock().unwrap()
                                .set_velocity([message[3], message[4], message[5]])
                                .unwrap();
                        }
                        Err(TryRecvError::Empty) => {

                        }
                        Err(TryRecvError::Disconnected) => {
                            println!("Inner Terminating");
                            txi.send(());
                            break;
                        }
                    }
                }
            });
            for decoding_result in decoder {
                if let Err(TryRecvError::Disconnected) =  rxi.try_recv() {
                    println!("Terminating");
                    break;
                }
                match decoding_result {
                    Err(e) => println!("Error: {:?}", e),
                    Ok(frame) => {
                        let samples: Vec<alto::Mono<i16>> = frame.samples[0].iter().map(|s| alto::Mono { center: s.to_i16() }).collect();
                        let buffer = crate::CONTEXT.new_buffer(samples, frame.sample_rate as i32).unwrap();
                        stream.lock().unwrap().queue_buffer(buffer).unwrap();
                        if stream.lock().unwrap().state() != alto::SourceState::Playing {
                            stream.lock().unwrap().play();
                        }
                        stream.lock().unwrap().unqueue_buffer();
                    },
                }
            }
        });
        SoundSource {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            time: SystemTime::now(),
            channel: tx,
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
        self.channel.send([self.position.x, self.position.y, self.position.z, self.velocity.x, self.position.y, self.position.z]);
    }

    pub fn get_position(&self) -> Vector3 {
        self.position
    }
    pub fn get_velocity(&self) -> Vector3 {
        self.velocity
    }
}

#[derive(Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }
}
impl Clone for Vector3 {
    fn clone(&self) -> Vector3 {
        *self
    }
}
