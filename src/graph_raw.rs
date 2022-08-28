#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum GraphType {
    Graph,
    Error,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GraphRaw {
    pub graph_type: GraphType,
    pub raw: *mut ::std::os::raw::c_void,
}

extern "C" {
    fn freeGraphRaw(gr: &mut GraphRaw);
}

impl Drop for GraphRaw {
    fn drop(&mut self) {
        unsafe {
            freeGraphRaw(self);
        }
    }
}
