#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate libc;

mod ffi;

#[cfg(test)]
mod tests {
    use super::ffi;
    use ::std::ffi::CString;

    #[test]
    fn it_works() {
        unsafe {
            let mut context: *mut ffi::cubeb = ::std::ptr::null_mut();
            let context_name = CString::new("cubeb-rs-test").unwrap().as_ptr();
            ffi::cubeb_init(&mut context as *mut *mut ffi::cubeb, context_name, ::std::ptr::null());
        }
    }
}
