use ffi;
use std::mem;

/**
 * Whether a particular device is an input device (e.g. a microphone), or an
 * output device (e.g. headphones). */
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceType {
    Unknown,
    Input,
    Output
}

impl From<ffi::cubeb_device_type> for DeviceType {
    fn from(other: ffi::cubeb_device_type) -> Self {
        unsafe { mem::transmute(other) }
    }
}

impl From<DeviceType> for ffi::cubeb_device_type {
    fn from(other: DeviceType) -> Self {
        unsafe { mem::transmute(other) }
    }
}
