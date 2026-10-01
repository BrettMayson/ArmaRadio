// Based on the original Live Radio implementation by BrettMayson.
// Playback and compatibility changes by Joncantplay.
use std::{
    path::Path,
    sync::{Arc, OnceLock},
};

use alto::Alto;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[cfg_attr(target_pointer_width = "32", folder = "resources/x86")]
#[cfg_attr(target_pointer_width = "64", folder = "resources/x64")]
struct Assets;

#[cfg(target_pointer_width = "32")]
const OPENAL_LIBRARY: &str = "live_radio_openal_x86.dll";

#[cfg(target_pointer_width = "64")]
const OPENAL_LIBRARY: &str = "live_radio_openal_x64.dll";

pub struct Audio;

impl Audio {
    pub fn get() -> Option<Arc<Alto>> {
        static AUDIO: OnceLock<Result<Arc<Alto>, String>> = OnceLock::new();

        match AUDIO.get_or_init(Self::initialize) {
            Ok(audio) => Some(audio.clone()),
            Err(message) => {
                error!("OpenAL initialization failed: {message}");
                None
            }
        }
    }

    fn initialize() -> Result<Arc<Alto>, String> {
        let embedded = Assets::get("OpenAL32.dll")
            .ok_or_else(|| "embedded OpenAL32.dll is missing".to_string())?;

        // A dedicated filename prevents another mod or a stale OpenAL32.dll in
        // the Arma directory from being loaded by accident.
        let openal_path = Path::new(OPENAL_LIBRARY);
        std::fs::write(openal_path, embedded.data.as_ref())
            .map_err(|error| format!("could not extract {}: {error}", openal_path.display()))?;

        let audio = Alto::load(openal_path)
            .map_err(|error| format!("could not load {}: {error}", openal_path.display()))?;

        Ok(Arc::new(audio))
    }
}
