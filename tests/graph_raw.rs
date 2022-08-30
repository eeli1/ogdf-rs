use ogdf_rs::graph_raw::GraphRaw;


#[test]
fn bindgen_test_layout_graph_raw() {
    assert_eq!(
        ::std::mem::size_of::<GraphRaw>(),
        16usize,
        concat!("Size of: ", stringify!(GraphRaw))
    );
    assert_eq!(
        ::std::mem::align_of::<GraphRaw>(),
        8usize,
        concat!("Alignment of ", stringify!(GraphRaw))
    );
    fn test_field_graph_type() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<GraphRaw>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).graph_type) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(GraphRaw),
                "::",
                stringify!(graph_type)
            )
        );
    }
    test_field_graph_type();
    fn test_field_raw() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<GraphRaw>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).raw) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(GraphRaw),
                "::",
                stringify!(raw)
            )
        );
    }
    test_field_raw();
}