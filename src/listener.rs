use std::{mem::MaybeUninit, sync::Arc};

use alto::{Context, DeviceObject};
use arma_rs::Group;

use crate::audio::Audio;

pub struct Listener;

impl Listener {
    pub fn get() -> Option<Arc<Context>> {
        static mut SINGLETON: MaybeUninit<Arc<Context>> = MaybeUninit::uninit();
        static mut INIT: bool = false;

        unsafe {
            if !INIT {
                SINGLETON.write(Arc::new({
                    let listener = {
                        let device = Audio::get()?.open(None).expect("can't open device");
                        debug!("{:?}", device.specifier());
                        device.new_context(None).expect("can't create context")
                    };
                    if listener.set_position([0.0, 0.0, 0.0]).is_err() {
                        error!("Error setting position");
                    };
                    if listener.set_velocity([0.0, 0.0, 0.0]).is_err() {
                        error!("Error setting velocity");
                    }
                    if listener
                        .set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]))
                        .is_err()
                    {
                        error!("Error setting orientation");
                    }
                    if listener.set_meters_per_unit(1.0).is_err() {
                        error!("Error setting meters per unit");
                    }
                    listener.set_distance_model(alto::DistanceModel::Exponent);
                    if listener.set_doppler_factor(0.2).is_err() {
                        error!("Error setting doppler factor");
                    };
                    listener
                }));
                INIT = true;
            }
            Some(SINGLETON.assume_init_ref().clone())
        }
    }
}

pub fn group() -> Group {
    Group::new().command("dir", command_set_orientation)
}

fn command_set_orientation(dx: f32, dy: f32, dz: f32, ux: f32, uy: f32, uz: f32) {
    let Some(listener) = Listener::get() else {
        return;
    };
    if let Err(e) = listener.set_orientation(([dx, dy, dz], [ux, uy, uz])) {
        error!("Error setting listener orientation: {}", e);
    }
}
