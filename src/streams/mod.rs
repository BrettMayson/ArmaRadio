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
mod ffmpeg;
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

    fn fail(&self, message: String) {
        if let Ok(senders) = self.0.read() {
            for sender in senders.iter() {
                let _ = sender.send(StreamPacket::Error(message.clone()));
            }
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
                    if error.starts_with("hls:") {
                        if let Err(error) = ffmpeg::decode(&url, &count, &senders) {
                            error!("Modern stream {url}: {error}");
                            senders.fail(error);
                        }
                        senders.close();
                        return;
                    }
                    error!("Failed to start stream {url}: {error}");
                    senders.fail(error);
                    senders.close();
                    return;
                }
            };

            if remote.is_aac() {
                drop(remote);
                if let Err(error) = ffmpeg::decode(&url, &count, &senders) {
                    error!("AAC stream {url}: {error}");
                    senders.fail(error);
                }
                senders.close();
                return;
            }

            let decoder = match Decoder::decode(remote) {
                Ok(decoder) => decoder,
                Err(error) => {
                    error!("Failed to create decoder for {url}: {error:?}");
                    senders.fail("decode: Could not initialize the MP3 decoder".to_string());
                    senders.close();
                    return;
                }
            };

            let mut invalid_frames = 0;
            let mut decoded_audio = false;
            for decoding_result in decoder {
                if count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
                    debug!("No listeners remain for {url}; stopping stream");
                    break;
                }

                let Ok(frame) = decoding_result else {
                    invalid_frames += 1;
                    if invalid_frames >= 100 {
                        senders.fail("unsupported: Unsupported codec or damaged MP3 stream".to_string());
                        break;
                    }
                    continue;
                };
                invalid_frames = 0;
                decoded_audio = true;
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
            if !decoded_audio && invalid_frames < 100 && count.load(std::sync::atomic::Ordering::Relaxed) > 0 {
                let message = "unsupported: No decodable MP3 audio was received".to_string();
                error!("Stream {url}: {message}");
                senders.fail(message);
            }
            senders.close();
        });
    }
}

pub enum StreamPacket {
    Data(Vec<alto::Mono<f32>>, i32),
    Title(String),
    AlbumArt(String),
    Error(String),
    Close,
    Check,
}

pub struct StreamListener {
    pub receiver: Receiver<StreamPacket>,
    pub count: Arc<AtomicU8>,
}

impl Drop for StreamListener {
    fn drop(&mut self) {
        let Ok(mut streams) = Streams::get().write() else {
            self.count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            error!("Stream map lock was poisoned while removing a listener");
            return;
        };
        // Remove the last listener and cached stream under the same lock. A retry
        // gets a fresh counter, so the old decoder cannot resume with the new one.
        self.count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        streams.retain(|_, stream| stream.count.load(std::sync::atomic::Ordering::SeqCst) > 0);
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
                    stream.senders.push(sender);
                    if stream
                        .count
                        .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
                        == 0
                    {
                        stream.start(&url);
                    }
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
