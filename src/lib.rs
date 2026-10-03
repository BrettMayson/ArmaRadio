// Based on the original Live Radio implementation by BrettMayson.
// Playback and compatibility changes by Joncantplay.
#![deny(clippy::unwrap_used)]

use std::{
    sync::{OnceLock, RwLock},
    time::{Duration, SystemTime},
};

use arma_rs::{arma, Extension};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[macro_use]
extern crate log;

pub mod album;
mod audio;
mod listener;
mod logger;
mod source;
mod streams;

#[arma]
pub fn init() -> Extension {
    let ext = Extension::build()
        .version(env!("CARGO_PKG_VERSION").to_string())
        .group("listener", listener::group())
        .group("source", source::group())
        .command("id", command_id)
        .command("heartbeat", command_heartbeat)
        .finish();
    logger::init(ext.context());

    std::thread::spawn(|| loop {
        if cfg!(test) {
            return;
        }
        let earlier = match Heartbeat::get().read() {
            Ok(earlier) => *earlier,
            Err(_) => {
                error!("Heartbeat lock was poisoned");
                source::cleanup();
                std::thread::sleep(Duration::from_secs(1));
                continue;
            }
        };
        let Ok(dur) = SystemTime::now().duration_since(earlier) else {
            error!("Error getting duration since last heartbeat");
            source::cleanup();
            continue;
        };

        if dur > Duration::from_secs(3) {
            source::cleanup();
        }
        std::thread::sleep(Duration::from_secs(1));
    });

    ext
}

fn command_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>()
        .to_lowercase()
}

fn command_heartbeat() {
    Heartbeat::beat();
}

pub struct Heartbeat;

impl Heartbeat {
    pub fn get() -> &'static RwLock<SystemTime> {
        static HEARTBEAT: OnceLock<RwLock<SystemTime>> = OnceLock::new();
        HEARTBEAT.get_or_init(|| RwLock::new(SystemTime::now()))
    }

    pub fn beat() {
        match Self::get().write() {
            Ok(mut heartbeat) => *heartbeat = SystemTime::now(),
            Err(_) => error!("Heartbeat lock was poisoned"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::init;

    #[test]
    fn radio1() {
        let ext = init().testing();
        let (id1, code) = ext.call("id", Some(Vec::new()));
        assert_eq!(code, 0);
        let (_, code) = ext.call(
            "source:new",
            Some(vec![
                id1.clone(),
                "http://pulseedm.cdnstream1.com:8124/1373_128".to_string(),
                "1.0".to_string(),
            ]),
        );
        assert_eq!(code, 0);
        println!("changing pos");
        let (_, code) = ext.call(
            "source:pos",
            Some(vec![
                id1.clone(),
                "1".to_string(),
                "0".to_string(),
                "0".to_string(),
            ]),
        );
        assert_eq!(code, 0);
        std::thread::sleep(std::time::Duration::from_secs(5));
        let (id2, code) = ext.call("id", Some(Vec::new()));
        assert_eq!(code, 0);
        let (_, code) = ext.call(
            "source:new",
            Some(vec![
                id2.clone(),
                "http://pulseedm.cdnstream1.com:8124/1373_128".to_string(),
                "1.0".to_string(),
            ]),
        );
        assert_eq!(code, 0);
        println!("changing pos");
        let (_, code) = ext.call(
            "source:pos",
            Some(vec![
                id2.clone(),
                "-1".to_string(),
                "0".to_string(),
                "0".to_string(),
            ]),
        );
        assert_eq!(code, 0);
        std::thread::sleep(std::time::Duration::from_secs(10));
        println!("destroying");
        let (_, code) = ext.call("source:destroy", Some(vec![id2]));
        assert_eq!(code, 0);
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("changing gain");
        let (_, code) = ext.call("source:gain", Some(vec![id1.clone(), "0.2".to_string()]));
        assert_eq!(code, 0);
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("changing pos");
        let (_, code) = ext.call(
            "source:pos",
            Some(vec![
                id1.clone(),
                "-1".to_string(),
                "0".to_string(),
                "0".to_string(),
            ]),
        );
        assert_eq!(code, 0);
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("destroying");
        let (_, code) = ext.call("source:destroy", Some(vec![id1]));
        assert_eq!(code, 0);
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("all done");
    }
}
