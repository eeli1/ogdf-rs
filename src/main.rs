pub mod error;
mod helper;
use helper::char_ptr_2_string;
pub mod graph_raw;
pub mod io;

use std::os::raw::c_char;

extern "C" {
    fn say_hello() -> *const c_char;
    fn test();
}

fn main() -> Result<(), error::Error> {
    println!("{}", char_ptr_2_string(unsafe { say_hello() })?);

    unsafe {
        test();
    }

    let graph_ml = r#"<?xml version="1.0"?>
    <graphml xmlns="http://graphml.graphdrawing.org/xmlns" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://graphml.graphdrawing.org/xmlns&#10;http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd">
        <graph id="G" edgedefault="directed">
            <node id="0" />
            <node id="1" />
            <edge id="0" source="0" target="1" />
        </graph>
    </graphml>"#;

    let gr = crate::io::reader::read_graph_ml(graph_ml.to_string())?;
    println!("{:?}", gr);
    // println!("{}", crate::io::writer::write_graph_ml(gr)?);

    Ok(())
}
