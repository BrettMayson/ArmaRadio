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
}

impl RemoteStream {
    pub fn new(url: &str, senders: Senders) -> Result<Self, String> {
        let response = Client::new()
            .get(url)
            .header("Icy-MetaData", "1")
            .send()
            .map_err(|e| e.to_string())?;
        if !response.status().is_success() {
            return Err(format!("HTTP {}", response.status()));
        }
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .to_ascii_lowercase();
        let interval = response
            .headers()
            .get("icy-metaint")
            .and_then(|i| i.to_str().ok())
            .and_then(|i| i.parse::<usize>().ok());
        let path = url.split('?').next().unwrap_or(url).to_ascii_lowercase();
        let mut response = BufReader::new(response);
        let adts = response
            .fill_buf()
            .map(|bytes| {
                bytes.len() >= 2
                    && bytes[0] == 0xff
                    && bytes[1] & 0xf0 == 0xf0
                    && bytes[1] & 0x06 == 0
            })
            .unwrap_or(false);
        let aac = content_type.contains("aac")
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

impl Read for RemoteStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if let Some(interval) = self.interval {
            let mut read = if buf.len() > interval - self.counter {
                let read = interval - self.counter;
                self.response
                    .read_exact(&mut buf[..interval - self.counter])?;
                self.counter += interval - self.counter;
                read
            } else {
                self.response.read_exact(buf)?;
                self.counter += buf.len();
                buf.len()
            };
            if self.counter == interval {
                let mut length = [0u8; 1];
                self.response.read_exact(&mut length)?;
                let length = length[0] as usize * 16;
                let mut metadata = vec![0u8; length];
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
            if read == 0 {
                read = self.read(buf)?;
            }
            Ok(read)
        } else {
            self.response.read(buf)
        }
    }
}
