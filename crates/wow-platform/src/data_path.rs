//! Default data directory published by the iOS host (Documents/WoW335).

use std::path::PathBuf;
use std::sync::Mutex;

static DEFAULT_DATA: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn set_default_data_path(path: PathBuf) {
    if let Ok(mut g) = DEFAULT_DATA.lock() {
        *g = Some(path);
    }
}

pub fn default_data_path() -> Option<PathBuf> {
    DEFAULT_DATA.lock().ok().and_then(|g| g.clone())
}

#[no_mangle]
pub extern "C" fn wow_ios_set_default_data_path(path: *const std::os::raw::c_char) {
    if path.is_null() {
        return;
    }
    let cstr = unsafe { std::ffi::CStr::from_ptr(path) };
    if let Ok(s) = cstr.to_str() {
        set_default_data_path(PathBuf::from(s));
        tracing::info!(%s, "default data path from iOS host");
    }
}
