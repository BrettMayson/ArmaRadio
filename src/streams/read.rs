// Based on the original Live Radio implementation by BrettMayson.
// Playback and compatibility changes by Joncantplay.
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

use regex::Regex;
use reqwest::blocking::{Client, Response};

use super::{Senders, StreamPacket};

pub struct RemoteStream {
    response: BufReader<Response>,
    counter: usize,
    interval: Option<usize>,
    regex: Regex,
    senders: Senders,
    aac: bool,
    last_track: Option<String>,
    read_failed: bool,
}

impl RemoteStream {
    pub fn new(url: &str, senders: Senders) -> Result<Self, String> {
        let response = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(15))
            .redirect(reqwest::redirect::Policy::limited(10))
            .build().map_err(|e| format!("connect: Could not initialize HTTP client: {e}"))?
            .get(url)
            .header("Icy-MetaData", "1")
            .send()
            .map_err(|e| {
                let category = if e.is_redirect() {"redirect"} else if e.is_timeout() {"timeout"} else {"connect"};
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
        if content_type.contains("text/html") || content_type.contains("audio/ogg") || content_type.contains("audio/flac") {
            return Err(format!("unsupported: Content type {content_type}"));
        }
        let interval = response
            .headers()
            .get("icy-metaint")
            .and_then(|i| i.to_str().ok())
            .and_then(|i| i.parse::<usize>().ok()).filter(|interval| *interval > 0);
        let mut response = BufReader::new(response);
        if response.fill_buf().map_err(|e| format!("read: {e}"))?.starts_with(b"#EXTM3U") {
            return Err("hls: HLS playlist".to_string());
        }
        let adts = response
            .fill_buf()
            .map(|bytes| {
                bytes.len() >= 2
                    && bytes[0] == 0xff
                    && bytes[1] & 0xf0 == 0xf0
                    && bytes[1] & 0x06 == 0
            })
            .map_err(|error| format!("read: Could not read stream header: {error}"))?;
        let aac = content_type.contains("aac") || content_type.contains("audio/mp4") || path.ends_with(".m4a") || path.ends_with(".mp4") || content_type.contains("video/mp4")
            || path.ends_with(".aac")
            || path.ends_with(".aacp")
            || adts;
        Ok(Self {
            interval,
            response,
            counter: 0,
            regex: Regex::new("(?m)StreamTitle='(.+?)';").map_err(|e| e.to_string())?,
            senders,
            aac,
            last_track: None,
            read_failed: false,
        })
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
        if buf.is_empty() { return Ok(0); }
        let Some(interval) = self.interval else { return self.response.read(buf); };
        if self.counter == interval {
            let mut length = [0u8; 1];
            match self.response.read_exact(&mut length) {
                Ok(()) => {},
                Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(0),
                Err(error) => return Err(error),
            }
            let mut metadata = vec![0u8; usize::from(length[0]) * 16];
            self.response.read_exact(&mut metadata)?;
            let metadata = String::from_utf8_lossy(&metadata);
            for cap in self.regex.captures_iter(&metadata) {
                let track = cap[1].to_string();
                if self.last_track.as_ref() == Some(&track) {continue;}
                self.last_track = Some(track.clone());
                let artwork_senders = self.senders.clone();
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
