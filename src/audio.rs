use std::{mem::MaybeUninit, sync::Arc};

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
        static mut SINGLETON: MaybeUninit<Arc<Alto>> = MaybeUninit::uninit();
        static mut INIT: bool = false;

        unsafe {
            if !INIT {
                SINGLETON.write(Arc::new({
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
                    Alto::load_default().expect("some sound exists")
                }));
                INIT = true;
            }
            Some(SINGLETON.assume_init_ref().clone())
        }
    }
}
