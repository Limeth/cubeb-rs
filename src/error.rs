use std;
use std::fmt;
use ffi;

#[derive(Debug)]
pub enum ErrorKind {
    General,
    InvalidFormat,
    InvalidParameter,
    NotSupported,
    DeviceUnavailable,
}

impl ErrorKind {
    pub fn from_code(other: i32) -> Option<Self> {
        match other {
            x if x == ffi::CUBEB_OK as i32 => None,
            x if x == ffi::CUBEB_ERROR as i32 => Some(ErrorKind::General),
            x if x == ffi::CUBEB_ERROR_INVALID_FORMAT as i32 => Some(ErrorKind::InvalidFormat),
            x if x == ffi::CUBEB_ERROR_INVALID_PARAMETER as i32 => Some(ErrorKind::InvalidParameter),
            x if x == ffi::CUBEB_ERROR_NOT_SUPPORTED as i32 => Some(ErrorKind::NotSupported),
            x if x == ffi::CUBEB_ERROR_DEVICE_UNAVAILABLE as i32 => Some(ErrorKind::DeviceUnavailable),
            code @ _ => panic!("An unexpected error Cubec occured: {}", code),
        }
    }
}

impl ErrorKind {
    pub fn description(&self) -> &'static str {
        match *self {
            ErrorKind::General => "Unclassified error.",
            ErrorKind::InvalidFormat => "Unsupported #cubeb_stream_params requested.",
            ErrorKind::InvalidParameter => "Invalid parameter specified.",
            ErrorKind::NotSupported => "Optional function not implemented in current backend.",
            ErrorKind::DeviceUnavailable => "Device specified by #cubeb_devid not available.",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[derive(Debug)]
pub enum Error {
    Cubeb(ErrorKind)
}

impl Error {
    pub fn cubeb(code: i32) -> Option<Self> {
        ErrorKind::from_code(code).map(Error::Cubeb)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Cubeb(ref kind) => write!(f, "Cubeb error: {}", kind),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            Error::Cubeb(ref kind) => kind.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Cubeb(_) => None,
        }
    }
}
