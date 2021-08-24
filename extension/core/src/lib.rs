use std::collections::HashMap;
use std::sync::Mutex;

use alto::{Alto, DeviceObject};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rust_embed::RustEmbed;

#[macro_use]
extern crate log;

mod source;
pub use source::SoundSource;

mod vector3;
pub use vector3::Vector3;

#[derive(RustEmbed)]
#[folder = "embed"]
struct Assets;

lazy_static::lazy_static! {
    static ref AL: Alto = {
        // OpenAL needs to live next to Arma
        let openal = std::path::Path::new("OpenAL32.dll");
        if !openal.exists() {
            let dll = Assets::get("OpenAL32.dll").unwrap();
            info!("Creating OpenAL.dll");
            let mut out = std::fs::File::create(&openal).unwrap_or_else(|_| {
                println!("Failed to create OpenAL32.dll");
                error!("Failed to create OpenAL32.dll");
                panic!("Failed to create OpenAL32.dll");
            });
            std::io::copy(&mut std::io::Cursor::new(dll.data), &mut out).unwrap_or_else(|_| {
                println!("Failed to write to OpenAL32.dll");
                error!("Failed to write to OpenAL32.dll");
                panic!("Failed to write to OpenAL32.dll");
            });
        }
        Alto::load_default().unwrap()
    };
    static ref SOURCES: Mutex<HashMap<String, SoundSource>> = Mutex::new(HashMap::new());
    static ref CONTEXT: alto::Context = {
        let device = AL.open(None).unwrap();
        debug!("{:?}", device.specifier());
        device.new_context(None).unwrap()
    };
}

pub static mut CALLBACK: Option<unsafe fn(*const i8, *const i8, *const i8)> = None;

pub fn init() {
    CONTEXT.set_position([0.0, 0.0, 0.0]).unwrap();
    CONTEXT.set_velocity([0.0, 0.0, 0.0]).unwrap();
    CONTEXT
        .set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]))
        .unwrap();
    CONTEXT.set_meters_per_unit(1.0).unwrap();
    CONTEXT.set_distance_model(alto::DistanceModel::Exponent);
    if CONTEXT.set_doppler_factor(0.2).is_err() {
        error!("Error setting doppler factor");
    };
}

pub fn cleanup() {
    let mut sources = SOURCES.lock().unwrap();
    let keys = sources
        .keys()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    for key in keys {
        sources.remove(&key);
    }
}

pub fn id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .collect::<String>()
        .to_lowercase()
}

pub fn create<S: Into<String>>(source: S, sid: S, gain: f32) {
    let sid = sid.into();
    SOURCES
        .lock()
        .unwrap()
        .insert(sid.clone(), SoundSource::new(sid, source.into(), gain));
}

pub fn destroy<S: Into<String>>(sid: S) -> bool {
    let sid = sid.into();
    info!("`{}` has been told to die", sid);
    if let Some(source) = SOURCES.lock().unwrap().remove(&sid) {
        info!("`{}` was playing `{}`", sid, source.station);
        true
    } else {
        false
    }
}

pub fn pos(sid: &str, x: f32, y: f32, z: f32) {
    if let Some(src) = SOURCES.lock().unwrap().get_mut(sid) {
        src.set_position([x, y, z]);
    }
}

pub fn gain(sid: &str, gain: f32) {
    if let Some(src) = SOURCES.lock().unwrap().get_mut(sid) {
        src.set_gain(gain);
    }
}

pub fn orientation(dx: f32, dy: f32, dz: f32, ux: f32, uy: f32, uz: f32) {
    CONTEXT
        .set_orientation(([dx, dy, dz], [ux, uy, uz]))
        .unwrap();
}

pub fn list() -> Vec<String> {
    SOURCES
        .lock()
        .unwrap()
        .keys()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}
