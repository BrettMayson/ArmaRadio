use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, SystemTime};

use alto::Alto;
use arma_rs::{rv, rv_callback, rv_handler};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rust_embed::RustEmbed;

mod source;
use crate::source::SoundSource;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

lazy_static! {
    static ref AL: Alto = {
        // OpenAL needs to live next to Arma
        let openal = std::path::Path::new("OpenAL32.dll");
        if !openal.exists() {
            // let mut resp = reqwest::blocking::get("https://github.com/Dynulo/ArmaRadio/releases/download/0.0/OpenAL32.dll").expect("request failed");
            // let mut out = std::fs::File::create(&openal).expect("failed to create file");
            // std::io::copy(&mut resp, &mut out).expect("failed to copy content");
            let dll = Assets::get("OpenAL32.dll").unwrap();
            info!("Creating OpenAL.dll");
            let mut out = std::fs::File::create(&openal).expect({
                println!("Failed to create OpenAL32.dll");
                error!("Failed to create OpenAL32.dll");
                "Failed to create OpenAL32.dll"
            });
            std::io::copy(&mut std::io::Cursor::new(dll), &mut out).expect({
                println!("Failed to write to OpenAL32.dll");
                error!("Failed to write to OpenAL32.dll");
                "Failed to write to OpenAL32.dll"
            });
        }
        Alto::load_default().unwrap()
    };
    static ref SOURCES: Mutex<HashMap<String, SoundSource>> = Mutex::new(HashMap::new());
    static ref CONTEXT: alto::Context = {
        let device = AL.open(None).unwrap();
        use alto::DeviceObject;
        debug!("{:?}", device.specifier());
        device.new_context(None).unwrap()
    };
}

static mut TIMESTAMP: Option<SystemTime> = None;

#[derive(RustEmbed)]
#[folder = "embed"]
struct Assets;

#[rv]
unsafe fn start() {
    TIMESTAMP = Some(SystemTime::now());
    thread::spawn(|| loop {
        let dur = SystemTime::now()
            .duration_since(TIMESTAMP.unwrap())
            .unwrap();
        if dur > Duration::from_secs(3) {
            let mut sources = SOURCES.lock().unwrap();
            let keys = sources
                .keys()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            for key in keys {
                sources.remove(&key);
            }
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
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .collect::<String>()
        .to_lowercase()
}

#[rv]
fn create(source: String, sid: String, gain: f32) -> String {
    SOURCES
        .lock()
        .unwrap()
        .insert(sid.clone(), SoundSource::new(source, gain));
    sid
}

#[rv]
fn destroy(sid: String) -> bool {
    info!("`{}` has been told to die", sid);
    if let Some(source) = SOURCES.lock().unwrap().remove(&sid) {
        info!("`{}` was playing `{}`", sid, source.station);
        true
    } else {
        false
    }
}

#[rv]
fn pos(sid: String, x: f32, y: f32, z: f32) {
    if let Some(src) = SOURCES.lock().unwrap().get_mut(&sid) {
        src.set_position([x, y, z]);
    }
}

#[rv]
fn gain(sid: String, gain: f32) {
    if let Some(src) = SOURCES.lock().unwrap().get_mut(&sid) {
        src.set_gain(gain);
    }
}

#[rv]
fn orientation(dx: f32, dy: f32, dz: f32, ux: f32, uy: f32, uz: f32) {
    CONTEXT
        .set_orientation(([dx, dy, dz], [ux, uy, uz]))
        .unwrap();
}

#[rv]
fn list() -> String {
    let sources = SOURCES
        .lock()
        .unwrap()
        .keys()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    format!("[{}]", sources.join(","))
}

use log::{Record, Level, LevelFilter, Metadata};
struct ArmaLogger;

impl log::Log for ArmaLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            rv_callback!("arma_radio_log", format!("{}", record.level()).to_lowercase(), format!("{}", record.args()));
        }
    }

    fn flush(&self) {}
}
static LOGGER: ArmaLogger = ArmaLogger;

#[rv_handler]
fn init() {
    if let Ok(()) = log::set_logger(&LOGGER) { log::set_max_level(LevelFilter::Info) }

    CONTEXT.set_position([0.0, 0.0, 0.0]).unwrap();
    CONTEXT.set_velocity([0.0, 0.0, 0.0]).unwrap();
    CONTEXT
        .set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]))
        .unwrap();
    CONTEXT.set_meters_per_unit(1.0).unwrap();
    CONTEXT.set_distance_model(alto::DistanceModel::Exponent);
    CONTEXT.set_doppler_factor(0.2);
}
