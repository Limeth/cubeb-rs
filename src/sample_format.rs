use ffi;
use std::mem;

/// You most likely want to use the `signed_16_native_endian` and `float_32_native_endian`
/// functions to retrieve the correct value for your system.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SampleFormat {
    Signed16LittleEndian,
    Signed16BigEndian,
    Float32LittleEndian,
    Float32BigEndian,
}

impl SampleFormat {
    #[cfg(target_endian = "little")]
    pub fn signed_16_native_endian() -> Self {
        SampleFormat::Signed16LittleEndian
    }

    #[cfg(target_endian = "big")]
    pub fn signed_16_native_endian() -> Self {
        SampleFormat::Signed16BigEndian
    }

    #[cfg(target_endian = "little")]
    pub fn float_32_native_endian() -> Self {
        SampleFormat::Float32LittleEndian
    }

    #[cfg(target_endian = "big")]
    pub fn float_32_native_endian() -> Self {
        SampleFormat::Float32BigEndian
    }
}

impl From<ffi::cubeb_sample_format> for SampleFormat {
    fn from(other: ffi::cubeb_sample_format) -> Self {
        unsafe { mem::transmute(other) }
    }
}

impl From<SampleFormat> for ffi::cubeb_sample_format {
    fn from(other: SampleFormat) -> Self {
        unsafe { mem::transmute(other) }
    }
}
