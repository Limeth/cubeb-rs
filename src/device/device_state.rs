use ffi;
use std::mem;

/**
 * Whether a particular device is an input device (e.g. a microphone), or an
 * output device (e.g. headphones). */
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceState {
    Disabled,
    Unplugged,
    Enabled,
}

impl From<ffi::cubeb_device_state> for DeviceState {
    fn from(other: ffi::cubeb_device_state) -> Self {
        unsafe { mem::transmute(other) }
    }
}

impl From<DeviceState> for ffi::cubeb_device_state {
    fn from(other: DeviceState) -> Self {
        unsafe { mem::transmute(other) }
    }
}
