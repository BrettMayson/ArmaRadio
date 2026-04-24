use std::sync::{Arc, OnceLock};

use alto::Alto;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources"]
struct Assets;

pub struct Audio();

// Raw Win32 declarations — avoids adding a new crate dependency.
#[cfg(target_os = "windows")]
mod win32 {
    pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 0x00000004;
    pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 0x00000002;

    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetModuleHandleExW(
            flags: u32,
            lp_module_name: *const u16,
            phmodule: *mut *mut std::ffi::c_void,
        ) -> i32;

        pub fn GetModuleFileNameW(
            hmodule: *mut std::ffi::c_void,
            lp_filename: *mut u16,
            n_size: u32,
        ) -> u32;
    }
}

/// Returns the directory that contains this extension DLL (live_radio_x64.dll).
///
/// Uses `GetModuleHandleExW` with `GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS` so
/// we don't need a `DllMain` — any function pointer inside our own DLL works
/// as the address hint.
fn dll_dir() -> Option<std::path::PathBuf> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use win32::*;

    unsafe {
        let mut hmodule: *mut std::ffi::c_void = std::ptr::null_mut();
        let ok = GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS
                | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            // Address of this function itself — guaranteed to be inside our DLL.
            dll_dir as *const u16,
            &mut hmodule,
        );
        if ok == 0 {
            error!("GetModuleHandleExW failed");
            return None;
        }

        let mut buf = vec![0u16; 32768];
        let len = GetModuleFileNameW(hmodule, buf.as_mut_ptr(), buf.len() as u32);
        if len == 0 {
            error!("GetModuleFileNameW failed");
            return None;
        }

        let path = std::path::PathBuf::from(OsString::from_wide(&buf[..len as usize]));
        path.parent().map(|p| p.to_path_buf())
    }
}

impl Audio {
    /// Gets a reference to the Alto (OpenAL) instance.
    ///
    /// Extracts `OpenAL32.dll` next to the extension DLL on first call if it
    /// is not already present there, then loads it via alto.
    pub fn get() -> Option<Arc<Alto>> {
        static SINGLETON: OnceLock<Option<Arc<Alto>>> = OnceLock::new();
        SINGLETON
            .get_or_init(|| {
                let dir = dll_dir()?;
                let openal = dir.join("OpenAL32.dll");

                if !openal.exists() {
                    let dll = Assets::get("OpenAL32.dll").expect("Failed to get OpenAL32.dll");
                    debug!("Extracting OpenAL32.dll to {:?}", openal);
                    let Ok(mut out) = std::fs::File::create(&openal) else {
                        error!("Failed to create {:?}", openal);
                        return None;
                    };
                    if std::io::copy(&mut std::io::Cursor::new(dll.data), &mut out).is_err() {
                        error!("Failed to write OpenAL32.dll to {:?}", openal);
                        return None;
                    }
                }

                match Alto::load_default() {
                    Ok(alto) => Some(Arc::new(alto)),
                    Err(e) => {
                        error!("Failed to load OpenAL: {}", e);
                        None
                    }
                }
            })
            .clone()
    }
}
