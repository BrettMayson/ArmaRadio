use std::{mem::MaybeUninit, sync::Arc};

use alto::{Context, DeviceObject};
use arma_rs::Group;

use crate::audio::Audio;

pub struct Listener;

impl Listener {
    pub fn get() -> Arc<Context> {
        static mut SINGLETON: MaybeUninit<Arc<Context>> = MaybeUninit::uninit();
        static mut INIT: bool = false;

        unsafe {
            if !INIT {
                SINGLETON.write(Arc::new({
                    let listener = {
                        let device = Audio::get().open(None).expect("can't open device");
                        println!("{:?}", device.specifier());
                        device.new_context(None).expect("can't create context")
                    };
                    listener
                        .set_position([0.0, 0.0, 0.0])
                        .expect("can't set position");
                    listener
                        .set_velocity([0.0, 0.0, 0.0])
                        .expect("can't set velocity");
                    listener
                        .set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]))
                        .expect("can't set orientation");
                    listener
                        .set_meters_per_unit(1.0)
                        .expect("can't set meters per unit");
                    listener.set_distance_model(alto::DistanceModel::Exponent);
                    if listener.set_doppler_factor(0.2).is_err() {
                        println!("Error setting doppler factor");
                    };
                    listener
                }));
                INIT = true;
            }
            SINGLETON.assume_init_ref().clone()
        }
    }
}

pub fn group() -> Group {
    Group::new().command("dir", command_set_orientation)
}

fn command_set_orientation(dx: f32, dy: f32, dz: f32, ux: f32, uy: f32, uz: f32) {
    if let Err(e) = Listener::get().set_orientation(([dx, dy, dz], [ux, uy, uz])) {
        println!("Error setting orientation: {}", e);
    }
}
