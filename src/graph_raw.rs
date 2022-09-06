#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GraphType {
    Graph = 0,
    Error = 1,
    Biconnec = 2,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GraphRaw {
    pub graph_type: GraphType,
    pub raw: *mut ::std::os::raw::c_void,
}
