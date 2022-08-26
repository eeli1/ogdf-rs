use std::ffi::CStr;
use std::os::raw::c_char;
use std::str;

extern "C" {
    fn say_hello() -> *const c_char;
    fn test();
}

fn main() {
    let c_buf: *const c_char = unsafe { say_hello() };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();
    println!("{}", str_buf);

    unsafe { test(); }
}
