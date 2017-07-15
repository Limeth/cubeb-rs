#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![feature(concat_idents)]

extern crate libc;

mod ffi;
pub mod error;
pub mod channel_layout;
pub mod sample_format;

pub use error::*;
pub use channel_layout::*;
pub use sample_format::*;

use std::mem;
use std::ffi::CStr;
use std::ffi::CString;

#[derive(Clone, Debug)]
pub struct StreamParams {
    raw: ffi::cubeb_stream_params,
}

impl StreamParams {
    pub fn new(format: SampleFormat, rate: u32, channels: u32, layout: ChannelLayout) -> Self {
        StreamParams {
            raw: ffi::cubeb_stream_params {
                format: format.into(),
                rate,
                channels,
                layout: layout.into(),
            }
        }
    }

    pub fn default(ctx: &Cubeb, format: SampleFormat) -> Result<StreamParams, Error> {
        Ok(StreamParams::new(
            format,
            ctx.get_preferred_sample_rate()?,
            ctx.get_max_channel_count()?,
            ctx.get_preferred_channel_layout()?,
        ))
    }
}

pub struct Cubeb {
    context: *mut ffi::cubeb,
}

macro_rules! getter_body {
    ($ffi_prefix:ident, $field:expr, $name:ident$($other_fields:tt)*) => {
        unsafe {
            use ffi::*; // Maybe proc macros can be used to avoid this
            let result = &mut mem::uninitialized() as *mut _;
            let error_code = concat_idents!($ffi_prefix, $name)($field$($other_fields)*, result);

            if let Some(error) = Error::cubeb(error_code) {
                Err(error)
            } else {
                Ok(*result)
            }
        }
    }
}
macro_rules! getter {
    ($ffi_prefix:ident, $field:ident, $name:ident, $ret:ty) => {
        pub fn $name(&self) -> Result<$ret, Error> {
            getter_body!($ffi_prefix, self.$field, $name)
        }
    }
}

impl Cubeb {
    pub fn new<T: Into<Vec<u8>>>(name: T) -> Result<Self, Error> {
        let mut context: *mut ffi::cubeb = ::std::ptr::null_mut();
        let context_name = CString::new(name).unwrap().as_ptr();

        unsafe {
            let error_code = ffi::cubeb_init(&mut context as *mut *mut ffi::cubeb, context_name, ::std::ptr::null());

            if let Some(error) = Error::cubeb(error_code) {
                Err(error)
            } else {
                Ok(Cubeb {
                    context,
                })
            }
        }
    }

    pub fn get_backend_id(&self) -> &str {
        unsafe {
            CStr::from_ptr(ffi::cubeb_get_backend_id(self.context))
                .to_str()
                .expect("The backend ID isn't a UTF-8 string for some reason.")
        }
    }

    pub fn get_min_latency(&self, stream_params: &StreamParams) -> Result<u32, Error> {
        let mut stream_params_raw = stream_params.raw.clone();
        getter_body!(cubeb_, self.context, get_min_latency, &mut stream_params_raw as *mut _)
    }

    pub fn get_preferred_channel_layout(&self) -> Result<ChannelLayout, Error> {
        getter_body!(cubeb_, self.context, get_preferred_channel_layout).map(|l| l.into())
    }

    getter!(cubeb_, context, get_max_channel_count, u32);
    getter!(cubeb_, context, get_preferred_sample_rate, u32);

    // Non-FFI bindings follow
    pub fn default_stream_params(&self, format: SampleFormat) -> Result<StreamParams, Error> {
        StreamParams::default(self, format)
    }
}

impl Drop for Cubeb {
    fn drop(&mut self) {
        unsafe {
            ffi::cubeb_destroy(self.context);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ctx = Cubeb::new("cubeb-rs-test").unwrap();
        let stream_params = ctx.default_stream_params(SampleFormat::float_32_native_endian());
        println!("get_backend_id: {}", ctx.get_backend_id());
        println!("get_max_channel_count: {:?}", ctx.get_max_channel_count());
        println!("get_preferred_sample_rate: {:?}", ctx.get_preferred_sample_rate());
        println!("get_preferred_channel_layout: {:?}", ctx.get_preferred_channel_layout());
        println!("default_stream_params: {:?}", stream_params);
        println!("get_min_latency: {:?}", ctx.get_min_latency(&stream_params.unwrap()));
    }
}
