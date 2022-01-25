use std::sync::RwLock;

use alto::{Alto, DeviceObject};
use arma_rs::Group;

use crate::Assets;

pub type Listener = alto::Context;

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
    pub static ref LISTENER: Listener = {
        let listener = {
            let device = AL.open(None).unwrap();
            debug!("{:?}", device.specifier());
            device.new_context(None).unwrap()
        };
        listener.set_position([0.0, 0.0, 0.0]).unwrap();
        listener.set_velocity([0.0, 0.0, 0.0]).unwrap();
        listener
            .set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]))
            .unwrap();
        listener.set_meters_per_unit(1.0).unwrap();
        listener.set_distance_model(alto::DistanceModel::Exponent);
        if listener.set_doppler_factor(0.2).is_err() {
            error!("Error setting doppler factor");
        };
        listener
    };
    pub static ref GAIN_MULTIPLIER: RwLock<f32> = RwLock::new(0.5);
}

pub fn cleanup() {
    // idk what to do here lol
}

pub fn group() -> Group {
    Group::new()
        // .command("pos", set_position)
        .command("dir", set_orientation)
        .command("gain", set_gain)
}

// fn set_position(x: f32, y: f32, z: f32) {
//     LISTENER.set_position([x, y, z]).unwrap();
// }

fn set_orientation(dx: f32, dy: f32, dz: f32, ux: f32, uy: f32, uz: f32) {
    LISTENER
        .set_orientation(([dx, dy, dz], [ux, uy, uz]))
        .unwrap();
}

fn set_gain(gain: f32) {
    *GAIN_MULTIPLIER.write().unwrap() = gain;
}
