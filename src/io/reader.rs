use crate::error::Error;
use crate::graph_raw::GraphRaw;
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn readGraphML(str: *const c_char) -> GraphRaw;
}

pub fn read_graph_ml(str: String) -> Result<GraphRaw, Error> {
    if let Ok(c_str) = CString::new(str) {
        let char_ptr: *const c_char = c_str.as_ptr() as *const c_char;
        unsafe { Ok(readGraphML(char_ptr)) }
    } else {
        Err(Error::NulError)
    }
}
