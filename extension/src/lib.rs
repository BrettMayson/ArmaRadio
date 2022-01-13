use arma_rs::{arma, Extension};
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

#[arma]
pub fn init() -> Extension {
    let ext = Extension::build()
        .group("listener", listener::group())
        .group("source", source::group())
        .finish();
    logger::init(ext.context());
    ext
}
