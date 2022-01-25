use std::{
    sync::RwLock,
    time::{Duration, SystemTime},
};

use arma_rs::{arma, Extension};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rust_embed::RustEmbed;

#[macro_use]
extern crate log;

mod listener;
mod logger;
mod source;
mod station;
mod vector3;

#[derive(RustEmbed)]
#[folder = "resources"]
struct Assets;

lazy_static::lazy_static! {
    static ref TIMESTAMP: RwLock<SystemTime> = RwLock::new(SystemTime::now());
}

#[arma]
pub fn init() -> Extension {
    let ext = Extension::build()
        .group("listener", listener::group())
        .group("source", source::group())
        .command("id", id)
        .command("heartbeat", heartbeat)
        .finish();
    logger::init(ext.context());

    std::thread::spawn(|| loop {
        let dur = SystemTime::now()
            .duration_since(*TIMESTAMP.read().unwrap())
            .unwrap();
        if dur > Duration::from_secs(3) {
            source::cleanup();
            listener::cleanup();
        }
        std::thread::sleep(Duration::from_secs(1));
    });

    ext
}

fn id() -> String {
    String::from_utf8(
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .collect::<Vec<u8>>(),
    )
    .unwrap()
    .to_lowercase()
}

fn heartbeat() {
    *TIMESTAMP.write().unwrap() = SystemTime::now();
}
