use std::sync::Mutex;
use std::thread;
use std::time::{Duration, SystemTime};

use arma_rs::{rv, rv_callback, rv_handler};

static mut TIMESTAMP: Option<SystemTime> = None;

lazy_static::lazy_static! {
    static ref GAIN_MULTIPLIER: Mutex<f32> = Mutex::new(0.5);
}

#[rv]
unsafe fn start() {
    TIMESTAMP = Some(SystemTime::now());
    thread::spawn(|| loop {
        let dur = SystemTime::now()
            .duration_since(TIMESTAMP.unwrap())
            .unwrap();
        if dur > Duration::from_secs(3) {
            live_radio::cleanup();
        }
        thread::sleep(Duration::from_secs(1));
    });
}

#[rv]
unsafe fn heartbeat() {
    TIMESTAMP = Some(SystemTime::now());
}

#[rv]
fn id() -> String {
    live_radio::id()
}

#[rv]
fn create(source: String, sid: String, gain: f32) {
    live_radio::create(source, sid, gain * *GAIN_MULTIPLIER.lock().unwrap());
}

#[rv]
fn destroy(sid: String) -> bool {
    live_radio::destroy(sid)
}

#[rv]
fn pos(sid: String, x: f32, y: f32, z: f32) {
    live_radio::pos(&sid, x, y, z)
}

#[rv]
fn gain(sid: String, gain: f32) {
    live_radio::gain(&sid, gain * *GAIN_MULTIPLIER.lock().unwrap())
}

#[rv]
fn gain_multiplier(gain: f32) {
    *GAIN_MULTIPLIER.lock().unwrap() = gain;
}


#[rv]
fn orientation(dx: f32, dy: f32, dz: f32, ux: f32, uy: f32, uz: f32) {
    live_radio::orientation(dx, dy, dz, ux, uy, uz)
}

#[rv]
fn list() -> String {
    format!("[{}]", live_radio::list().join(","))
}

use log::{Level, LevelFilter, Metadata, Record};
struct ArmaLogger;

impl log::Log for ArmaLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            rv_callback!(
                "live_radio_log",
                format!("{}", record.level()).to_lowercase(),
                format!("{}", record.args())
            );
        }
    }

    fn flush(&self) {}
}
static LOGGER: ArmaLogger = ArmaLogger;

#[rv_handler]
fn init() {
    if let Ok(()) = log::set_logger(&LOGGER) {
        log::set_max_level(LevelFilter::Info)
    }
    live_radio::init();
}
