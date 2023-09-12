use std::io::{BufReader, Read};

use regex::Regex;
use reqwest::blocking::{Client, Response};

use super::{Senders, StreamPacket};

pub struct RemoteStream {
    response: BufReader<Response>,
    counter: usize,
    interval: Option<usize>,
    regex: Regex,
    senders: Senders,
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
                    for sender in self.senders.0.read().expect("not poisoned").iter() {
                        let _ = sender.send(StreamPacket::Title(cap[1].to_string()));
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
