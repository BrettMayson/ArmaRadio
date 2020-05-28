use std::collections::HashMap;
use std::sync::Mutex;

use alto::Alto;
use arma_rs::{rv, rv_handler};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

mod source;
use crate::source::SoundSource;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref AL: Alto = Alto::load_default().unwrap();
    static ref SOURCES: Mutex<HashMap<String, SoundSource>> = Mutex::new(HashMap::new());
    static ref CONTEXT: alto::Context = {
        let device = AL.open(None).unwrap();
        device.new_context(None).unwrap()
    };
}

#[rv]
fn id() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(8).collect()
}

#[rv]
fn create(source: String) -> String {
    let id = id();
    SOURCES.lock().unwrap().insert(id.clone(), SoundSource::new(source));
    id
}

#[rv]
fn destroy(id: String) -> bool {
    SOURCES.lock().unwrap().remove(&id).is_some()
}

#[rv]
fn pos(id: String, x: f32, y: f32, z: f32) {
    if let Some(src) = SOURCES.lock().unwrap().get_mut(&id) {
        src.set_position([x, y, z]);
    }
    println!("Set Pos");
}

#[rv_handler]
fn init() {
    CONTEXT.set_position([0.0, 0.0, 0.0]).unwrap();
    CONTEXT.set_velocity([0.0, 0.0, 0.0]).unwrap();
    CONTEXT
        .set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]))
        .unwrap();
}
