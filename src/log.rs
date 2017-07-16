use ffi;
use std::mem;
use error::Error;
use std::option::Option;
use std::os::raw::c_char;
use std::ffi::CStr;
use std::sync::RwLock;

lazy_static! {
    static ref STRING_CALLBACK: RwLock<Option<fn(&str)>> = RwLock::new(None);
}

extern "C" {
    fn set_log_callback_c(log_level: ffi::cubeb_log_level, string_callback: Option<extern "C" fn(formatted: *const c_char)>) -> i32;
}

extern "C" fn inner_string_callback(formatted: *const c_char) {
    let string_callback_read = STRING_CALLBACK.read().expect("Could not lock the `STRING_CALLBACK` for reading.");

    if let Some(ref function) = string_callback_read.as_ref() {
        unsafe {
            let string = CStr::from_ptr(formatted).to_string_lossy();

            function(&string);
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Disabled,
    Normal,
    Verbose,
}

impl From<ffi::cubeb_log_level> for LogLevel {
    fn from(other: ffi::cubeb_log_level) -> Self {
        unsafe { mem::transmute(other) }
    }
}

impl From<LogLevel> for ffi::cubeb_log_level {
    fn from(other: LogLevel) -> Self {
        unsafe { mem::transmute(other) }
    }
}

pub fn set_log_callback(log_level: LogLevel, callback: Option<fn(&str)>) -> Result<(), Error> {
    let some = callback.is_some();
    *STRING_CALLBACK.write().expect("Could not lock `STRING_CALLBACK` for writing.") = callback;

    unsafe {
        let inner_string_callback: Option<extern "C" fn(*const i8)> = if some {
            Some(inner_string_callback)
        } else {
            None
        };

        let error_code = set_log_callback_c(log_level.into(), inner_string_callback);

        if let Some(error) = Error::cubeb(error_code) {
            Err(error)
        } else {
            Ok(())
        }
    }
}
