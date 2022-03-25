use std::io::Read;

use arma_rs::Context;
use regex::Regex;
use reqwest::blocking::Response;

pub struct Station {
    id: String,
    request: Response,
    counter: usize,
    interval: Option<usize>,
    initial: bool,
    ctx: Context,
}
impl Station {
    pub const fn new(ctx: Context, request: Response, id: String) -> Self {
        Self {
            id,
            request,
            counter: 0,
            interval: None,
            initial: true,
            ctx,
        }
    }
}
impl Read for Station {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.initial {
            self.initial = false;
            if let Some(interval) = self.request.headers().get("icy-metaint") {
                self.interval = Some(interval.to_str().unwrap().parse::<usize>().unwrap());
            }
        }
        let mut ret = self.request.read(buf)?;
        if let Some(i) = self.interval {
            self.counter += ret;
            if self.counter > i {
                let index = i - (self.counter - ret);
                let length = buf[index] as usize * 16_usize;
                if index + 1 + length >= buf.len() {
                    error!("Metadata is cut off");
                } else {
                    let metadata = String::from_utf8_lossy(&buf[(index + 1)..(index + 1 + length)]);
                    lazy_static::lazy_static! {
                        static ref RE_STREAM_TITLE: Regex = Regex::new("(?m)StreamTitle='(.+?)';").unwrap();
                    }
                    for cap in RE_STREAM_TITLE.captures_iter(&metadata) {
                        info!("Received title: {:?}", cap[1].to_string());
                        self.ctx.callback(
                            "live_radio",
                            "title",
                            Some(vec![self.id.to_string(), cap[1].to_string()]),
                        );
                    }
                    if ret - length - 1 - index == 0 {
                        self.counter = ret - length - 1 - index;
                        if ret == 1 {
                            ret = self.read(buf)?;
                        }
                    } else {
                        for b in index..ret - length - 1 {
                            buf[b] = buf[b + length + 1];
                        }
                        ret = ret - length - 1;
                        self.counter = ret - index;
                    }
                }
            }
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::init;

    #[test]
    fn radio1() {
        let ext = init().testing();
        unsafe {
            let (id, code) = ext.call("id", Some(Vec::new()));
            assert_eq!(code, 0);
            let (_, code) = ext.call(
                "source:new",
                Some(vec![
                    id.clone(),
                    "http://stream.live.vc.bbcmedia.co.uk/bbc_radio_one".to_string(),
                    "1.0".to_string(),
                ]),
            );
            assert_eq!(code, 0);
            std::thread::sleep(std::time::Duration::from_secs(3));
            let (_, code) = ext.call("source:gain", Some(vec![id.clone(), "0.2".to_string()]));
            assert_eq!(code, 0);
            std::thread::sleep(std::time::Duration::from_secs(3));
            let (_, code) = ext.call(
                "source:pos",
                Some(vec![
                    id.clone(),
                    "1".to_string(),
                    "1".to_string(),
                    "1".to_string(),
                ]),
            );
            assert_eq!(code, 0);
            std::thread::sleep(std::time::Duration::from_secs(3));
            let (_, code) = ext.call("source:destroy", Some(vec![id]));
            assert_eq!(code, 0);
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    }
}
