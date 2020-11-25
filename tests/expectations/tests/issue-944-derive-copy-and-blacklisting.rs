#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub struct BlacklistMe(u8);

/// Because this type contains a blacklisted type, it should not derive Copy.
#[repr(C, packed)]
pub struct ShouldNotBeCopy {
    pub a: BlacklistMe,
}
#[test]
fn bindgen_test_layout_ShouldNotBeCopy() {
    assert_eq!(
        ::std::mem::size_of::<ShouldNotBeCopy>(),
        1usize,
        concat!("Size of: ", stringify!(ShouldNotBeCopy))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldNotBeCopy>(),
        1usize,
        concat!("Alignment of ", stringify!(ShouldNotBeCopy))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldNotBeCopy>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldNotBeCopy),
            "::",
            stringify!(a)
        )
    );
}
impl Default for ShouldNotBeCopy {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
