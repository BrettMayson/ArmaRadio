// Based on the original Live Radio implementation by BrettMayson.
// Playback and compatibility changes by Joncantplay.
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock, atomic::AtomicUsize},
};

use crossbeam_channel::{Receiver, Sender};
use simplemad::Decoder;

use self::read::RemoteStream;

#[cfg(test)]
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
    pub count: Arc<AtomicUsize>,
    pub senders: Senders,
    pub finished: Arc<std::sync::atomic::AtomicBool>,
}

impl Stream {
    pub fn start(&self, url: &str) {
        debug!("Starting stream: {url}");
        let count = self.count.clone();
        let url = url.to_string();
        let senders = self.senders.clone();
        let finished = self.finished.clone();

        std::thread::spawn(move || {
            struct Finish(Arc<std::sync::atomic::AtomicBool>, Senders);
            impl Drop for Finish {
                fn drop(&mut self) {
                    self.0.store(true, std::sync::atomic::Ordering::SeqCst);
                    if std::thread::panicking() {
                        self.1.fail("decode: Stream worker panicked".to_string());
                    }
                    self.1.close();
                }
            }
            let _finish = Finish(finished, senders.clone());
            if count.load(std::sync::atomic::Ordering::SeqCst) == 0 {
                return;
            }
            let remote = match RemoteStream::new(&url, senders.clone(), count.clone()) {
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

                let frame = match decoding_result {
                    Ok(frame) => frame,
                    Err(simplemad::SimplemadError::Read(error)) => {
                        senders.fail(format!("read: MP3 network read failed: {error}"));
                        break;
                    }
                    Err(simplemad::SimplemadError::EOF) => break,
                    Err(simplemad::SimplemadError::Mad(error)) => {
                        // libmad handles incomplete input internally. Only actual
                        // corrupt/sync frames reach this branch.
                        if (error as u32) < 0x0100 {
                            senders.fail(format!("decode: Fatal MP3 decoder error: {error:?}"));
                            break;
                        }
                        invalid_frames += 1;
                        if invalid_frames == 1 {
                            debug!("MP3 resynchronizing: {error:?}");
                        }
                        if invalid_frames >= 100 {
                            senders.fail("decode: Repeated invalid MP3 frames".to_string());
                            break;
                        }
                        continue;
                    }
                };
                invalid_frames = 0;
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

                if samples.is_empty() {
                    continue;
                }
                if !decoded_audio {
                    debug!("Initial MP3 audio frame decoded: {url}");
                }
                decoded_audio = true;
                if !senders.send_samples(samples, frame.sample_rate as i32) {
                    break;
                }
            }
            if !decoded_audio
                && invalid_frames < 100
                && count.load(std::sync::atomic::Ordering::Relaxed) > 0
            {
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
    #[cfg_attr(test, allow(dead_code))]
    AlbumArt(String),
    Error(String),
    Close,
    Check,
}

pub struct StreamListener {
    pub receiver: Receiver<StreamPacket>,
    pub count: Arc<AtomicUsize>,
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

        // Serialize lookup + listener registration + insertion. Concurrent
        // source workers must not overwrite one another's cached stream.
        let mut streams = match Self::get().write() {
            Ok(streams) => streams,
            Err(_) => {
                let _ = sender.send(StreamPacket::Error(
                    "decode: Stream map unavailable".to_string(),
                ));
                return StreamListener {
                    receiver,
                    count: Arc::new(AtomicUsize::new(1)),
                };
            }
        };
        if let Some(stream) = streams
            .get(&url)
            .filter(|s| !s.finished.load(std::sync::atomic::Ordering::SeqCst))
        {
            stream.senders.push(sender);
            stream
                .count
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            return StreamListener {
                receiver,
                count: stream.count.clone(),
            };
        }

        debug!("Creating new stream for {url}");
        let stream = Stream {
            count: Arc::new(AtomicUsize::new(1)),
            finished: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            senders: Senders(Arc::new(RwLock::new(vec![sender]))),
        };
        stream.start(&url);
        let listener = StreamListener {
            receiver,
            count: stream.count.clone(),
        };

        streams.insert(url, stream);

        listener
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "manual external station smoke test"]
    fn it_works() {
        let receiver =
            super::Streams::listen("http://pulseedm.cdnstream1.com:8124/1373_128".to_string());
        std::thread::sleep(std::time::Duration::from_secs(3));
        drop(receiver);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}

#[cfg(test)]
mod fragmented_mp3_tests {
    use simplemad::Decoder;
    use std::io::{self, Cursor, Read};
    struct Fragments {
        data: Cursor<&'static [u8]>,
        lengths: Vec<usize>,
        step: usize,
    }
    impl Read for Fragments {
        fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
            let n = out.len().min(self.lengths[self.step % self.lengths.len()]);
            self.step += 1;
            self.data.read(&mut out[..n])
        }
    }
    fn decode(lengths: Vec<usize>) -> Vec<i32> {
        let reader = Fragments {
            data: Cursor::new(include_bytes!("../../tests/fixtures/mp3_stereo.mp3")),
            lengths,
            step: 0,
        };
        let decoder = Decoder::decode(reader).expect("decoder");
        let mut pcm = Vec::new();
        for frame in decoder {
            match frame {
                Ok(frame) => pcm.extend(frame.samples[0].iter().map(simplemad::MadFixed32::to_raw)),
                Err(simplemad::SimplemadError::Mad(_)) => {}
                Err(error) => panic!("valid fragmented MP3 failed: {error:?}"),
            }
        }
        assert!(pcm.len() > 120000);
        pcm
    }
    #[test]
    fn fragmented_mp3_matches_contiguous_pcm() {
        let whole = decode(vec![32768]);
        assert!(
            decode(vec![180, 350, 700, 1200]) == whole,
            "fragmented PCM differs"
        );
        assert!(decode(vec![1]) == whole, "one-byte PCM differs");
        assert!(
            decode(vec![417, 3, 8192, 5]) == whole,
            "frame-boundary PCM differs"
        );
    }
}
