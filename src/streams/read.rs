use std::io::{BufReader, Read};

use regex::Regex;
use reqwest::blocking::{Client, Response};

use crate::album::search_album;

use super::{Senders, StreamPacket};

pub struct RemoteStream {
    response: BufReader<Response>,
    counter: usize,
    interval: Option<usize>,
    regex: Regex,
    senders: Senders,
    last_track: Option<String>,
}

impl RemoteStream {
    pub fn new(url: &str, senders: Senders) -> Result<Self, String> {
        let response = Client::new()
            .get(url)
            .header("Icy-MetaData", "1")
            .send()
            .map_err(|e| e.to_string())?;
        Ok(Self {
            interval: response
                .headers()
                .get("icy-metaint")
                .and_then(|i| i.to_str().ok())
                .and_then(|i| i.parse::<usize>().ok()),
            response: BufReader::new(response),
            counter: 0,
            regex: Regex::new("(?m)StreamTitle='(.+?)';").map_err(|e| e.to_string())?,
            senders,
            last_track: None,
        })
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
                    if self.last_track != Some(cap[1].to_string()) {
                        for sender in self.senders.0.read().expect("not poisoned").iter() {
                            let _ = sender.send(StreamPacket::Title(cap[1].to_string()));
                        }
                        self.last_track = Some(cap[1].to_string());
                        let track = cap[1].to_string();
                        let senders = self.senders.clone();
                        std::thread::spawn(move || {
                            if let Some(path) = search_album(&track) {
                                for sender in senders.0.read().expect("not poisoned").iter() {
                                    let _ = sender.send(StreamPacket::AlbumArt(path.clone()));
                                }
                            }
                        });
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
