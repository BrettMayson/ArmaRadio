use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock, atomic::AtomicU8},
};

use crossbeam_channel::{Receiver, Sender};
use symphonia::core::{
    audio::SampleBuffer,
    codecs::DecoderOptions,
    formats::FormatOptions,
    io::{MediaSource, MediaSourceStream, MediaSourceStreamOptions},
    meta::MetadataOptions,
    probe::Hint,
};

use self::hls::HlsSource;
use self::read::RemoteStream;

mod hls;
mod read;

/// HLS streams are served as `.m3u8` playlists rather than a single continuous byte stream.
fn is_hls_url(url: &str) -> bool {
    url.split(['?', '#'])
        .next()
        .unwrap_or(url)
        .to_lowercase()
        .ends_with(".m3u8")
}

#[derive(Clone)]
pub struct Senders(pub Arc<RwLock<Vec<Sender<StreamPacket>>>>);

impl Senders {
    pub fn push(&self, sender: Sender<StreamPacket>) {
        self.0.write().expect("not poisoned").push(sender);
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
            let (source, hint): (Box<dyn MediaSource>, Hint) = if is_hls_url(&url) {
                match HlsSource::new(&url) {
                    Ok(hls) => {
                        let hint = hls.hint();
                        (Box::new(hls), hint)
                    }
                    Err(e) => {
                        error!("Failed to start HLS stream: {e}");
                        for sender in senders.0.read().expect("not poisoned").iter() {
                            let _ = sender.send(StreamPacket::Close);
                        }
                        return;
                    }
                }
            } else {
                let remote = RemoteStream::new(&url, senders.clone());
                let Ok(remote) = remote else {
                    error!(
                        "Failed to start stream: {}",
                        remote.err().expect("error expected")
                    );
                    return;
                };

                let mut hint = Hint::new();
                if let Some(content_type) = remote.content_type() {
                    let content_type = content_type.to_lowercase();
                    if content_type.contains("aac") {
                        hint.with_extension("aac");
                    } else if content_type.contains("mpeg") || content_type.contains("mp3") {
                        hint.with_extension("mp3");
                    }
                }
                (Box::new(remote), hint)
            };

            let mss = MediaSourceStream::new(source, MediaSourceStreamOptions::default());
            let probed = symphonia::default::get_probe().format(
                &hint,
                mss,
                &FormatOptions::default(),
                &MetadataOptions::default(),
            );
            let Ok(mut probed) = probed else {
                error!("Failed to probe stream: {url}");
                for sender in senders.0.read().expect("not poisoned").iter() {
                    let _ = sender.send(StreamPacket::Close);
                }
                return;
            };

            let Some(track) = probed.format.default_track().cloned() else {
                error!("Failed to find a track in stream: {url}");
                for sender in senders.0.read().expect("not poisoned").iter() {
                    let _ = sender.send(StreamPacket::Close);
                }
                return;
            };
            let track_id = track.id;

            let Ok(mut decoder) =
                symphonia::default::get_codecs().make(&track.codec_params, &DecoderOptions::default())
            else {
                error!("Failed to create decoder for stream: {url}");
                for sender in senders.0.read().expect("not poisoned").iter() {
                    let _ = sender.send(StreamPacket::Close);
                }
                return;
            };

            let mut sample_buf: Option<SampleBuffer<f32>> = None;

            loop {
                if count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
                    debug!("no listeners, shutting down stream");
                    break;
                }
                let Ok(packet) = probed.format.next_packet() else {
                    break;
                };
                if packet.track_id() != track_id {
                    continue;
                }
                let Ok(buffer) = decoder.decode(&packet) else {
                    continue; // error!("Error: {:?}", e),
                };

                let spec = *buffer.spec();
                let channels = spec.channels.count();
                let buf = sample_buf
                    .get_or_insert_with(|| SampleBuffer::new(buffer.capacity() as u64, spec));
                buf.copy_interleaved_ref(buffer);
                let interleaved = buf.samples();

                let mut samples: Vec<alto::Mono<f32>> = Vec::new();
                for frame in interleaved.chunks(channels) {
                    let center = if channels >= 2 {
                        f32::midpoint(frame[0], frame[1])
                    } else {
                        frame[0]
                    };
                    samples.push(alto::Mono { center });
                }

                let mut delete = false;
                for sender in senders.0.read().expect("not poisoned").iter() {
                    if let Err(e) = sender.send(StreamPacket::Data(
                        samples.clone(),
                        spec.rate.cast_signed(),
                    )) {
                        error!("Failed to send data: {e}");
                        delete = true;
                    }
                }
                if delete {
                    senders
                        .0
                        .write()
                        .expect("not poisoned")
                        .retain(|s| s.send(StreamPacket::Check).is_ok());
                }
            }
        });
    }
}


pub enum StreamPacket {
    Data(Vec<alto::Mono<f32>>, i32),
    AlbumArt(String),
    Title(String),
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
        Streams::get()
            .write()
            .expect("not poisoned")
            .iter()
            .for_each(|(_, stream)| {
                stream
                    .senders
                    .0
                    .write()
                    .expect("not poisoned")
                    .retain(|s| s.send(StreamPacket::Check).is_ok());
            });
    }
}

pub struct Streams;

impl Streams {
    pub fn get() -> Arc<RwLock<HashMap<String, Stream>>> {
        static SINGLETON: OnceLock<Arc<RwLock<HashMap<String, Stream>>>> = OnceLock::new();
        SINGLETON
            .get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
            .clone()
    }

    pub fn listen(url: String) -> StreamListener {
        let (sender, receiver) = crossbeam_channel::unbounded();
        if let Some(stream) = Self::get().read().expect("not poisoned").get(&url) {
            debug!("using existing stream for {url}");
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
        debug!("creating new stream for {url}");
        let stream = Stream {
            count: Arc::new(AtomicU8::new(1)),
            senders: Senders(Arc::new(RwLock::new(vec![sender]))),
        };
        stream.start(&url);
        let sl = StreamListener {
            receiver,
            count: stream.count.clone(),
        };
        Self::get()
            .write()
            .expect("not poisoned")
            .insert(url, stream);
        sl
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pulse_edm() {
        let receiver =
            super::Streams::listen("http://pulseedm.cdnstream1.com:8124/1373_128".to_string());
        std::thread::sleep(std::time::Duration::from_secs(3));
        drop(receiver);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    #[test]
    fn classic_rock() {
        let receiver =
            super::Streams::listen("http://listen.classicrock109.com:10042".to_string());
        std::thread::sleep(std::time::Duration::from_secs(3));
        drop(receiver);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    #[test]
    fn aac() {
        let receiver = 
            super::Streams::listen("http://hirschmilch.de:7000/stream/5/".to_string());
        std::thread::sleep(std::time::Duration::from_secs(3));
        drop(receiver);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    #[test]
    fn hls() {
        let receiver =
            super::Streams::listen("http://as-hls-ww-live.akamaized.net/pool_01505109/live/ww/bbc_radio_one/bbc_radio_one.isml/bbc_radio_one-audio%3d96000.norewind.m3u8".to_string());
        std::thread::sleep(std::time::Duration::from_secs(3));
        drop(receiver);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
