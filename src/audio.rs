use std::sync::{Arc, OnceLock};

use alto::Alto;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources"]
struct Assets;

pub struct Audio();

impl Audio {
    /// Gets a reference to the NATS connection.
    ///
    /// # Panics
    ///
    /// Panics if the NATS connection can not be initialized.
    pub fn get() -> Option<Arc<Alto>> {
        static SINGLETON: OnceLock<Arc<Alto>> = OnceLock::new();

        if let Some(alto) = SINGLETON.get() {
            return Some(alto.clone());
        }

        let openal = std::path::Path::new("OpenAL32.dll");
        if !openal.exists() {
            let dll = Assets::get("OpenAL32.dll").expect("Failed to get OpenAL32.dll");
            debug!("Creating OpenAL.dll");
            let Ok(mut out) = std::fs::File::create(openal) else {
                error!("Failed to create OpenAL32.dll");
                return None;
            };
            if std::io::copy(&mut std::io::Cursor::new(dll.data), &mut out).is_err() {
                error!("Failed to write to OpenAL32.dll");
                return None;
            }
        }

        let alto = Arc::new(Alto::load_default().expect("some sound exists"));
        Some(SINGLETON.get_or_init(|| alto).clone())
    }
}
