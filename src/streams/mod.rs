// Based on the original Live Radio implementation by BrettMayson.
// Playback and compatibility changes by Joncantplay.
use std::{
    collections::HashMap,
    sync::{atomic::AtomicU8, Arc, OnceLock, RwLock},
};

use crossbeam_channel::{Receiver, Sender};
use simplemad::Decoder;

use self::read::RemoteStream;

mod aac;
mod read;

#[derive(Clone)]
pub struct Senders(pub Arc<RwLock<Vec<Sender<StreamPacket>>>>);

impl Senders {
    pub fn push(&self, sender: Sender<StreamPacket>) {
        match self.0.write() {
            Ok(mut senders) => senders.push(sender),
            Err(_) => error!("Stream sender lock was poisoned"),
        }
    }

    fn close(&self) {
        let Ok(senders) = self.0.read() else {
            error!("Stream sender lock was poisoned while closing a stream");
            return;
        };
        for sender in senders.iter() {
            let _ = sender.send(StreamPacket::Close);
        }
    }

    fn send_samples(&self, samples: Vec<alto::Mono<f32>>, sample_rate: i32) -> bool {
        let Ok(senders) = self.0.read() else {
            error!("Stream sender lock was poisoned while sending audio");
            return false;
        };
        let mut remove_closed = false;
        for sender in senders.iter() {
            if sender
                .send(StreamPacket::Data(samples.clone(), sample_rate))
                .is_err()
            {
                remove_closed = true;
            }
        }
        drop(senders);

        if remove_closed {
            match self.0.write() {
                Ok(mut senders) => {
                    senders.retain(|sender| sender.send(StreamPacket::Check).is_ok())
                }
                Err(_) => return false,
            }
        }
        true
    }
}

pub struct Stream {
    pub count: Arc<AtomicU8>,
    pub senders: Senders,
}

impl Stream {
    pub fn start(&self, url: &str) {
        debug!("Starting stream: {url}");
        let count = self.count.clone();
        let url = url.to_string();
        let senders = self.senders.clone();

        std::thread::spawn(move || {
            let remote = match RemoteStream::new(&url, senders.clone()) {
                Ok(remote) => remote,
                Err(error) => {
                    error!("Failed to start stream {url}: {error}");
                    senders.close();
                    return;
                }
            };

            if remote.is_aac() {
                if let Err(error) = aac::decode(remote, &count, &senders) {
                    error!("AAC stream {url}: {error}");
                }
                senders.close();
                return;
            }

            let decoder = match Decoder::decode(remote) {
                Ok(decoder) => decoder,
                Err(error) => {
                    error!("Failed to create decoder for {url}: {error:?}");
                    senders.close();
                    return;
                }
            };

            for decoding_result in decoder {
                if count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
                    debug!("No listeners remain for {url}; stopping stream");
                    break;
                }

                let Ok(frame) = decoding_result else {
                    continue;
                };
                let Some(left) = frame.samples.first() else {
                    continue;
                };
                let right = frame.samples.get(1).unwrap_or(left);

                let samples: Vec<alto::Mono<f32>> = left
                    .iter()
                    .zip(right.iter())
                    .map(|(left, right)| alto::Mono {
                        center: (left.to_f32() + right.to_f32()) / 2.0_f32,
                    })
                    .collect();

                if !senders.send_samples(samples, frame.sample_rate as i32) {
                    break;
                }
            }
            senders.close();
        });
    }
}

pub enum StreamPacket {
    Data(Vec<alto::Mono<f32>>, i32),
    Title(String),
    AlbumArt(String),
    Close,
    Check,
}

pub struct StreamListener {
    pub receiver: Receiver<StreamPacket>,
    pub count: Arc<AtomicU8>,
}

impl Drop for StreamListener {
    fn drop(&mut self) {
        self.count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);

        let Ok(streams) = Streams::get().read() else {
            error!("Stream map lock was poisoned while removing a listener");
            return;
        };
        for stream in streams.values() {
            match stream.senders.0.write() {
                Ok(mut senders) => {
                    senders.retain(|sender| sender.send(StreamPacket::Check).is_ok());
                }
                Err(_) => error!("Stream sender lock was poisoned while removing a listener"),
            }
        }
    }
}

pub struct Streams;

impl Streams {
    pub fn get() -> &'static RwLock<HashMap<String, Stream>> {
        static STREAMS: OnceLock<RwLock<HashMap<String, Stream>>> = OnceLock::new();
        STREAMS.get_or_init(|| RwLock::new(HashMap::new()))
    }

    pub fn listen(url: String) -> StreamListener {
        let (sender, receiver) = crossbeam_channel::unbounded();

        match Self::get().read() {
            Ok(streams) => {
                if let Some(stream) = streams.get(&url) {
                    debug!("Using existing stream for {url}");
                    if stream
                        .count
                        .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
                        == 0
                    {
                        stream.start(&url);
                    }
                    stream.senders.push(sender);
                    return StreamListener {
                        receiver,
                        count: stream.count.clone(),
                    };
                }
            }
            Err(_) => error!("Stream map lock was poisoned while finding a stream"),
        }

        debug!("Creating new stream for {url}");
        let stream = Stream {
            count: Arc::new(AtomicU8::new(1)),
            senders: Senders(Arc::new(RwLock::new(vec![sender]))),
        };
        stream.start(&url);
        let listener = StreamListener {
            receiver,
            count: stream.count.clone(),
        };

        match Self::get().write() {
            Ok(mut streams) => {
                streams.insert(url, stream);
            }
            Err(_) => error!("Stream map lock was poisoned while creating a stream"),
        }

        listener
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let receiver =
            super::Streams::listen("http://pulseedm.cdnstream1.com:8124/1373_128".to_string());
        std::thread::sleep(std::time::Duration::from_secs(3));
        drop(receiver);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
