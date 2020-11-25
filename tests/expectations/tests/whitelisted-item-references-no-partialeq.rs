#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NoPartialEq {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_NoPartialEq() {
    assert_eq!(
        ::std::mem::size_of::<NoPartialEq>(),
        1usize,
        concat!("Size of: ", stringify!(NoPartialEq))
    );
    assert_eq!(
        ::std::mem::align_of::<NoPartialEq>(),
        1usize,
        concat!("Alignment of ", stringify!(NoPartialEq))
    );
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WhitelistMe {
    pub a: NoPartialEq,
}
#[test]
fn bindgen_test_layout_WhitelistMe() {
    assert_eq!(
        ::std::mem::size_of::<WhitelistMe>(),
        1usize,
        concat!("Size of: ", stringify!(WhitelistMe))
    );
    assert_eq!(
        ::std::mem::align_of::<WhitelistMe>(),
        1usize,
        concat!("Alignment of ", stringify!(WhitelistMe))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WhitelistMe>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WhitelistMe),
            "::",
            stringify!(a)
        )
    );
}
