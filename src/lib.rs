#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![feature(concat_idents)]

extern crate libc;

mod ffi;
pub mod error;

use std::mem;
use std::ffi::CStr;
use std::ffi::CString;
use error::Error;

pub struct Cubeb {
    context: *mut ffi::cubeb,
}

macro_rules! getter_body {
    ($ffi_prefix:ident, $field:expr, $name:ident) => {
        unsafe {
            use ffi::*; // Maybe proc macros can be used to avoid this
            let result = &mut mem::uninitialized() as *mut _;
            let error_code = concat_idents!($ffi_prefix, $name)($field, result);

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
    pub fn new<T: Into<Vec<u8>>>(name: T) -> Self {
        let mut context: *mut ffi::cubeb = ::std::ptr::null_mut();
        let context_name = CString::new(name).unwrap().as_ptr();

        unsafe {
            ffi::cubeb_init(&mut context as *mut *mut ffi::cubeb, context_name, ::std::ptr::null());
        }

        Cubeb {
            context,
        }
    }

    pub fn get_backend_id(&self) -> &str {
        unsafe {
            CStr::from_ptr(ffi::cubeb_get_backend_id(self.context))
                .to_str()
                .expect("The backend ID isn't a UTF-8 string for some reason.")
        }
    }

    getter!(cubeb_, context, get_max_channel_count, u32);
    getter!(cubeb_, context, get_preferred_sample_rate, u32);
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
        let ctx = Cubeb::new("cubeb-rs-test");
        println!("get_backend_id: {}", ctx.get_backend_id());
        println!("get_max_channel_count: {:?}", ctx.get_max_channel_count());
        println!("get_preferred_sample_rate: {:?}", ctx.get_preferred_sample_rate());
    }
}
