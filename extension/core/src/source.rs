use std::io::{self, Read};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

use alto::Source;
use regex::Regex;
use reqwest::blocking::{Client, Response};

// mpeg
use simplemad::Decoder;

use crate::Vector3;

struct OnlineRadio {
    request: Response,
    counter: usize,
    interval: Option<usize>,
    initial: bool,
}
impl OnlineRadio {
    fn new(request: Response) -> Self {
        Self {
            request,
            counter: 0,
            interval: None,
            initial: true,
        }
    }
}
impl Read for OnlineRadio {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.initial {
            self.initial = false;
            // println!("Headers: {:?}", self.request.headers());
            if let Some(interval) = self.request.headers().get("icy-metaint") {
                self.interval = Some(interval.to_str().unwrap().parse::<usize>().unwrap());
                // println!("Interval is set to: {:?}", self.interval);
            }
        }
        let mut ret = self.request.read(buf)?;
        if let Some(i) = self.interval {
            self.counter += ret;
            if self.counter > i {
                // println!("Counter: {}, Ret: {}", self.counter, ret);
                let index = i - (self.counter - ret);
                let length = buf[index] as usize * 16usize;
                // println!("Metadata Length: {}", length);
                if index + 1 + length >= buf.len() {
                    error!("Metadata is cut off");
                } else {
                    let metadata = String::from_utf8_lossy(&buf[(index + 1)..(index + 1 + length)]);
                    lazy_static::lazy_static! {
                        static ref RE_STREAM_TITLE: Regex = Regex::new("(?m)StreamTitle='(.+?)';").unwrap();
                    }
                    for cap in RE_STREAM_TITLE.captures_iter(&metadata) {
                        // println!("Title: {}", &cap[1]);
                        // arma_rs::rv_callback!("arma_radio", self.id.clone(), cap[1].to_string());
                        unsafe {
                            if let Some(f) = &mut crate::CALLBACK {
                                f(cap[1].to_string());
                            }
                        }
                    }
                    if ret - length - 1 - index != 0 {
                        // println!("Moving {:?} items", (index..ret-length-1));
                        for b in index..ret - length - 1 {
                            buf[b] = buf[b + length + 1];
                        }
                        ret = ret - length - 1;
                        self.counter = ret - index;
                    } else {
                        self.counter = ret - length - 1 - index;
                        if ret == 1 {
                            ret = self.read(buf)?;
                        }
                    }
                }
            }
        }
        Ok(ret)
    }
}

#[derive(Debug, Clone)]
pub struct SoundSource {
    position: Vector3,
    velocity: Vector3,
    gain: f32,
    time: SystemTime,
    channel: Sender<[f32; 7]>,
    pub station: String,
}
impl SoundSource {
    pub fn new<S: Into<String>>(station: S, gain: f32) -> Self {
        let (tx, rx): (Sender<[f32; 7]>, Receiver<[f32; 7]>) = mpsc::channel();
        let station = station.into();
        let s = station.clone();
        std::thread::spawn(move || {
            let client = Client::new();
            info!("Starting Radio. URL: {}", station);
            let mut request = client.get(&station);
            request = request.header("Icy-MetaData", "1");
            let decoder = Decoder::decode(OnlineRadio::new(request.send().unwrap())).unwrap();
            let stream = Arc::new(Mutex::new(crate::CONTEXT.new_streaming_source().unwrap()));
            if stream
                .lock()
                .unwrap()
                .set_soft_spatialization(alto::SoftSourceSpatialization::Enabled)
                .is_err()
            {
                warn!("Error setting soft spatialization");
            }
            if stream.lock().unwrap().set_max_gain(2f32).is_err() {
                warn!("Error setting max gain");
            };
            if stream.lock().unwrap().set_gain(gain).is_err() {
                warn!("Error setting gain");
            };
            // stream.lock().unwrap().set_rolloff_factor(1.0);
            let (txi, rxi): (Sender<()>, Receiver<()>) = mpsc::channel();
            let inner_stream = stream.clone();
            thread::spawn(move || loop {
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
                if let Err(TryRecvError::Disconnected) = rxi.try_recv() {
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
                                    / 2.0f32,
                            });
                        }
                        let buffer = if let Ok(mut buffer) = stream.lock().unwrap().unqueue_buffer()
                        {
                            if buffer.set_data(samples, frame.sample_rate as i32).is_err() {
                                warn!("Error setting buffer sample data");
                            }
                            buffer
                        } else {
                            crate::CONTEXT
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
        SoundSource {
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

    pub fn get_position(&self) -> Vector3 {
        self.position
    }
    pub fn get_velocity(&self) -> Vector3 {
        self.velocity
    }
}
