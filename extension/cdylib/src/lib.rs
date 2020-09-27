use std::thread;
use std::time::{Duration, SystemTime};

use arma_rs::{rv, rv_callback, rv_handler};

static mut TIMESTAMP: Option<SystemTime> = None;

#[rv]
unsafe fn start() {
    TIMESTAMP = Some(SystemTime::now());
    thread::spawn(|| loop {
        let dur = SystemTime::now()
            .duration_since(TIMESTAMP.unwrap())
            .unwrap();
        if dur > Duration::from_secs(3) {
            dynulo_radio::cleanup();
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
    dynulo_radio::id()
}

#[rv]
fn create(source: String, sid: String, gain: f32) {
    dynulo_radio::create(source, sid, gain);
}

#[rv]
fn destroy(sid: String) -> bool {
    dynulo_radio::destroy(sid)
}

#[rv]
fn pos(sid: String, x: f32, y: f32, z: f32) {
    dynulo_radio::pos(&sid, x, y, z)
}

#[rv]
fn gain(sid: String, gain: f32) {
    dynulo_radio::gain(&sid, gain)
}

#[rv]
fn orientation(dx: f32, dy: f32, dz: f32, ux: f32, uy: f32, uz: f32) {
    dynulo_radio::orientation(dx, dy, dz, ux, uy, uz)
}

#[rv]
fn list() -> String {
    format!("[{}]", dynulo_radio::list().join(","))
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
                "arma_radio_log",
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
    dynulo_radio::init();
}
