extern crate libc;
use libc::c_char;
use std::ffi::CStr;
use std::str;

// #[link(name="COIN")]
// #[link(name="OGDF")]
// #[link(name="c_wrapper")]
extern "C"{
    // fn double_input(input: libc::c_int) -> libc::c_int;
    fn say_hello() -> *const c_char;
    // fn test();
    // fn randomDouble( m:f64,sd:f64 ) -> f64;
}

fn main() {
    // let input = 4;
    // let output = unsafe { double_input(input) };
    // println!("{} * 2 = {}", input, output);
    // 
    let c_buf: *const c_char = unsafe { say_hello() };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();  // if necessary
    
    println!("{}",str_buf);
    // 
    // unsafe { test(); }
    // unsafe { let _t = randomDouble(4.0,6.0); }
}
