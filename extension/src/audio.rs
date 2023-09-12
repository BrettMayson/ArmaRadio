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
    pub fn get() -> Arc<Alto> {
        static mut SINGLETON: MaybeUninit<Arc<Alto>> = MaybeUninit::uninit();
        static mut INIT: bool = false;

        unsafe {
            if !INIT {
                SINGLETON.write(Arc::new({
                    let openal = std::path::Path::new("OpenAL32.dll");
                    if !openal.exists() {
                        let dll = Assets::get("OpenAL32.dll").expect("Failed to get OpenAL32.dll");
                        println!("Creating OpenAL.dll");
                        let mut out = std::fs::File::create(openal).unwrap_or_else(|_| {
                            println!("Failed to create OpenAL32.dll");
                            println!("Failed to create OpenAL32.dll");
                            panic!("Failed to create OpenAL32.dll");
                        });
                        std::io::copy(&mut std::io::Cursor::new(dll.data), &mut out)
                            .unwrap_or_else(|_| {
                                println!("Failed to write to OpenAL32.dll");
                                println!("Failed to write to OpenAL32.dll");
                                panic!("Failed to write to OpenAL32.dll");
                            });
                    }
                    Alto::load_default().expect("some sound exists")
                }));
                INIT = true;
            }
            SINGLETON.assume_init_ref().clone()
        }
    }
}
