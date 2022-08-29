extern crate libc;

use crate::error::{make_msg, Error};
use std::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;

#[inline]
pub fn char_ptr_2_string(char_ptr: *const c_char) -> Result<String, Error> {
    let c_str = unsafe { CStr::from_ptr(char_ptr) };
    if let Ok(str_slice) = c_str.to_str() {
        let str_buf = str_slice.to_owned();
        unsafe {
            libc::free(char_ptr as *mut c_void);
        }
        Ok(str_buf)
    } else {
        Err(make_msg("convert CStr failed"))
    }
}
