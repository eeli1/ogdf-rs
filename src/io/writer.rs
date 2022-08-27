use crate::error::Error;
use crate::graph_raw::GraphRaw;
use crate::helper::char_ptr_2_string;
use std::os::raw::c_char;

extern "C" {
    fn writeChaco(gr: GraphRaw) -> *const c_char;
    fn writeGraphML(gr: GraphRaw) -> *const c_char;

}

pub fn write_chaco(gr: GraphRaw) -> Result<String, Error> {
    char_ptr_2_string(unsafe { writeChaco(gr) })
}

pub fn write_graph_ml(gr: GraphRaw) -> Result<String, Error> {
    char_ptr_2_string(unsafe { writeGraphML(gr) })
}
