// Based on the original Live Radio implementation by BrettMayson.
// Playback and compatibility changes by Joncantplay.
use std::sync::{Arc, OnceLock};

use alto::{Context, DeviceObject};
use arma_rs::Group;

use crate::audio::Audio;

pub struct Listener;

impl Listener {
    pub fn get() -> Option<Arc<Context>> {
        static LISTENER: OnceLock<Result<Arc<Context>, String>> = OnceLock::new();

        match LISTENER.get_or_init(Self::initialize) {
            Ok(listener) => Some(listener.clone()),
            Err(message) => {
                error!("Audio device initialization failed: {message}");
                None
            }
        }
    }

    fn initialize() -> Result<Arc<Context>, String> {
        let audio = Audio::get().ok_or_else(|| "OpenAL is unavailable".to_string())?;
        let device = audio
            .open(None)
            .map_err(|error| format!("could not open the default playback device: {error}"))?;

        debug!("Using playback device: {:?}", device.specifier());

        let listener = device
            .new_context(None)
            .map_err(|error| format!("could not create the OpenAL context: {error}"))?;

        listener
            .set_position([0.0, 0.0, 0.0])
            .map_err(|error| format!("could not set listener position: {error}"))?;
        listener
            .set_velocity([0.0, 0.0, 0.0])
            .map_err(|error| format!("could not set listener velocity: {error}"))?;
        listener
            .set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]))
            .map_err(|error| format!("could not set listener orientation: {error}"))?;
        listener
            .set_meters_per_unit(1.0)
            .map_err(|error| format!("could not set listener scale: {error}"))?;
        listener.set_distance_model(alto::DistanceModel::Exponent);
        listener
            .set_doppler_factor(0.0)
            .map_err(|error| format!("could not set listener Doppler factor: {error}"))?;

        Ok(Arc::new(listener))
    }
}

pub fn group() -> Group {
    Group::new().command("dir", command_set_orientation)
}

fn command_set_orientation(dx: f32, dy: f32, dz: f32, ux: f32, uy: f32, uz: f32) {
    let Some(listener) = Listener::get() else {
        return;
    };
    if let Err(error) = listener.set_orientation(([dx, dy, dz], [ux, uy, uz])) {
        error!("Error setting listener orientation: {error}");
    }
}
