#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct A {
    _unused: [u8; 0],
}
#[repr(C)]
#[repr(align(1))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct B {
    pub _bindgen_opaque_blob: u8,
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(
        ::std::mem::size_of::<B>(),
        1usize,
        concat!("Size of: ", stringify!(B))
    );
    assert_eq!(
        ::std::mem::align_of::<B>(),
        1usize,
        concat!("Alignment of ", stringify!(B))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN1B1aE"]
    pub static mut B_a: A;
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct C {
    pub b: B,
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(
        ::std::mem::size_of::<C>(),
        1usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        1usize,
        concat!("Alignment of ", stringify!(C))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<C>())).b as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(C), "::", stringify!(b))
    );
}
