// Based on the original Live Radio implementation by BrettMayson.
// Playback and compatibility changes by Joncantplay.
use std::io::{BufReader, Read, Seek, SeekFrom};

use regex::Regex;
use reqwest::blocking::Client;
use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use super::{Senders, StreamPacket};

pub struct RemoteStream {
    response: BufReader<Box<dyn Read + Send + Sync>>,
    prefix: VecDeque<u8>,
    count: Arc<AtomicUsize>,
    counter: usize,
    interval: Option<usize>,
    regex: Regex,
    senders: Senders,
    aac: bool,
    last_track: Option<String>,
    read_failed: bool,
}

impl RemoteStream {
    pub fn new(url: &str, senders: Senders, count: Arc<AtomicUsize>) -> Result<Self, String> {
        let response = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(10))
            .timeout(std::time::Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .map_err(|e| format!("connect: Could not initialize HTTP client: {e}"))?
            .get(url)
            .header("Icy-MetaData", "1")
            .send()
            .map_err(|e| {
                let category = if e.is_redirect() {
                    "redirect"
                } else if e.is_timeout() {
                    "timeout"
                } else {
                    "connect"
                };
                format!("{category}: {e}")
            })?;
        if !response.status().is_success() {
            return Err(format!("http: HTTP {}", response.status()));
        }
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .to_ascii_lowercase();
        let path = response.url().path().to_ascii_lowercase();
        if content_type.contains("mpegurl") || path.ends_with(".m3u8") {
            return Err("hls: HLS playlist".to_string());
        }
        if content_type.contains("text/html")
            || content_type.contains("audio/ogg")
            || content_type.contains("audio/flac")
        {
            return Err(format!("unsupported: Content type {content_type}"));
        }
        let interval = response
            .headers()
            .get("icy-metaint")
            .and_then(|i| i.to_str().ok())
            .and_then(|i| i.parse::<usize>().ok())
            .filter(|interval| *interval > 0);
        let aac = content_type.contains("aac")
            || content_type.contains("audio/mp4")
            || path.ends_with(".m4a")
            || path.ends_with(".mp4")
            || content_type.contains("video/mp4")
            || path.ends_with(".aac")
            || path.ends_with(".aacp");
        debug!("HTTP connection established: {url}");
        let mut remote = Self {
            interval,
            response: BufReader::new(Box::new(response)),
            prefix: VecDeque::new(),
            count,
            counter: 0,
            regex: Regex::new("(?m)StreamTitle='(.+?)';").map_err(|e| e.to_string())?,
            senders,
            aac,
            last_track: None,
            read_failed: false,
        };
        // Sniff audio AFTER removing ICY metadata. Read across packet boundaries
        // and preserve every sniffed byte for the decoder.
        let mut prefix = [0_u8; 7];
        let mut length = 0;
        while length < prefix.len() {
            let read = remote
                .read_audio(&mut prefix[length..])
                .map_err(|e| format!("read: {e}"))?;
            if read == 0 {
                break;
            }
            length += read;
        }
        if prefix[..length].starts_with(b"#EXTM3U") {
            return Err("hls: HLS playlist".to_string());
        }
        remote.aac |= length >= 2 && prefix[0] == 0xff && prefix[1] & 0xf6 == 0xf0;
        remote.prefix.extend(&prefix[..length]);
        Ok(remote)
    }

    pub fn is_aac(&self) -> bool {
        self.aac
    }
}

impl Seek for RemoteStream {
    fn seek(&mut self, _pos: SeekFrom) -> std::io::Result<u64> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "live stream is not seekable",
        ))
    }
}

impl symphonia::core::io::MediaSource for RemoteStream {
    fn is_seekable(&self) -> bool {
        false
    }

    fn byte_len(&self) -> Option<u64> {
        None
    }
}

impl RemoteStream {
    fn read_audio(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.is_empty() || self.count.load(Ordering::SeqCst) == 0 {
            return Ok(0);
        }
        let Some(interval) = self.interval else {
            return self.response.read(buf);
        };
        if self.counter == interval {
            let mut length = [0u8; 1];
            match self.response.read_exact(&mut length) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(0),
                Err(error) => return Err(error),
            }
            let mut metadata = vec![0u8; usize::from(length[0]) * 16];
            self.response.read_exact(&mut metadata)?;
            let metadata = String::from_utf8_lossy(&metadata);
            for cap in self.regex.captures_iter(&metadata) {
                let track = cap[1].to_string();
                if self.last_track.as_ref() == Some(&track) {
                    continue;
                }
                self.last_track = Some(track.clone());
                #[cfg(not(test))]
                let artwork_senders = self.senders.clone();
                #[cfg(not(test))]
                std::thread::spawn(move || {
                    if let Some(path) = crate::album::search_album(&track) {
                        if let Ok(senders) = artwork_senders.0.read() {
                            for sender in senders.iter() {
                                let _ = sender.send(StreamPacket::AlbumArt(path.clone()));
                            }
                        }
                    }
                });
                if let Ok(senders) = self.senders.0.read() {
                    for sender in senders.iter() {
                        let _ = sender.send(StreamPacket::Title(cap[1].to_string()));
                    }
                } else {
                    error!("Stream sender lock was poisoned while sending metadata");
                }
            }
            self.counter = 0;
        }
        let length = buf.len().min(interval - self.counter);
        let read = self.response.read(&mut buf[..length])?;
        self.counter += read;
        Ok(read)
    }
}

impl Read for RemoteStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if !self.prefix.is_empty() {
            let length = buf.len().min(self.prefix.len());
            for byte in &mut buf[..length] {
                if let Some(value) = self.prefix.pop_front() {
                    *byte = value;
                }
            }
            return Ok(length);
        }
        let result = self.read_audio(buf);
        if let Err(error) = &result {
            if !self.read_failed {
                self.read_failed = true;
                let message = format!("read: Stream connection interrupted: {error}");
                error!("{message}");
                self.senders.fail(message);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::sync::RwLock;
    struct Tiny(Cursor<Vec<u8>>);
    impl Read for Tiny {
        fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
            let n = out.len().min(1);
            self.0.read(&mut out[..n])
        }
    }
    #[test]
    fn icy_boundaries_preserve_every_audio_byte() {
        let audio = include_bytes!("../../tests/fixtures/mp3_stereo.mp3");
        let interval = 180;
        let mut wire = Vec::new();
        for (index, bytes) in audio.chunks(interval).enumerate() {
            wire.extend_from_slice(bytes);
            if bytes.len() == interval {
                if index % 2 == 0 {
                    let mut metadata = b"StreamTitle='Test Artist - Test Song';".to_vec();
                    metadata.resize(48, 0);
                    wire.push(3);
                    wire.extend(metadata);
                } else {
                    wire.push(0);
                }
            }
        }
        let (tx, rx) = crossbeam_channel::unbounded();
        let mut remote = RemoteStream {
            response: BufReader::new(Box::new(Tiny(Cursor::new(wire)))),
            prefix: VecDeque::new(),
            count: Arc::new(AtomicUsize::new(1)),
            counter: 0,
            interval: Some(interval),
            regex: Regex::new("(?m)StreamTitle='(.+?)';").expect("regex"),
            senders: Senders(Arc::new(RwLock::new(vec![tx]))),
            aac: false,
            last_track: None,
            read_failed: false,
        };
        let mut output = Vec::new();
        remote.read_to_end(&mut output).expect("read ICY");
        assert_eq!(output, audio);
        let titles: Vec<_> = rx
            .try_iter()
            .filter_map(|packet| match packet {
                StreamPacket::Title(title) => Some(title),
                _ => None,
            })
            .collect();
        assert_eq!(titles, vec!["Test Artist - Test Song"]);
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;
    use std::{io::Write, net::TcpListener, sync::RwLock};
    fn server(
        status: &str,
        body: Vec<u8>,
        content_type: &str,
    ) -> (String, std::thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("test socket");
        let url = format!("http://{}/stream", listener.local_addr().expect("address"));
        let header = format!(
            "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        let thread = std::thread::spawn(move || {
            let (mut socket, _) = listener.accept().expect("accept");
            socket
                .set_read_timeout(Some(std::time::Duration::from_secs(2)))
                .expect("timeout");
            let mut request = [0; 4096];
            let _ = socket.read(&mut request);
            socket.write_all(header.as_bytes()).expect("header");
            // Deliberately split sniffing and MP3 frames over slow, tiny writes.
            for chunk in body.chunks(180) {
                if socket.write_all(chunk).is_err() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        });
        (url, thread)
    }
    fn senders() -> Senders {
        Senders(Arc::new(RwLock::new(Vec::new())))
    }
    #[test]
    fn fragmented_http_mp3_starts_without_retry() {
        let (url, worker) = server(
            "200 OK",
            include_bytes!("../../tests/fixtures/mp3_stereo.mp3").to_vec(),
            "audio/mpeg",
        );
        let remote =
            RemoteStream::new(&url, senders(), Arc::new(AtomicUsize::new(1))).expect("HTTP");
        assert!(!remote.is_aac());
        let decoder = simplemad::Decoder::decode(remote).expect("decoder");
        let samples: usize = decoder
            .filter_map(Result::ok)
            .map(|f| f.samples[0].len())
            .sum();
        worker.join().expect("HTTP thread");
        assert!(samples > 120000);
    }
    #[test]
    fn http_failure_is_immediate() {
        let (url, worker) = server("503 Service Unavailable", vec![], "text/plain");
        let started = std::time::Instant::now();
        let result = RemoteStream::new(&url, senders(), Arc::new(AtomicUsize::new(1)));
        assert!(matches!(result, Err(ref message) if message.starts_with("http:")));
        assert!(started.elapsed() < std::time::Duration::from_secs(2));
        worker.join().expect("HTTP thread");
    }
    #[test]
    fn aac_sniff_survives_fragmented_header() {
        let (url, worker) = server(
            "200 OK",
            include_bytes!("../../tests/fixtures/aac_lc_stereo.adts").to_vec(),
            "application/octet-stream",
        );
        let remote =
            RemoteStream::new(&url, senders(), Arc::new(AtomicUsize::new(1))).expect("HTTP");
        assert!(remote.is_aac());
        drop(remote);
        worker.join().expect("HTTP thread");
    }
}
