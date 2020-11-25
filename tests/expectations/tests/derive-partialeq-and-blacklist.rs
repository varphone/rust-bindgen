#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub struct BlacklistMe(u8);

/// Because this type contains a blacklisted type, it should not derive
/// PartialEq.
#[repr(C, packed)]
pub struct ShouldNotDerivePartialEq {
    pub a: BlacklistMe,
}
#[test]
fn bindgen_test_layout_ShouldNotDerivePartialEq() {
    assert_eq!(
        ::std::mem::size_of::<ShouldNotDerivePartialEq>(),
        1usize,
        concat!("Size of: ", stringify!(ShouldNotDerivePartialEq))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldNotDerivePartialEq>(),
        1usize,
        concat!("Alignment of ", stringify!(ShouldNotDerivePartialEq))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldNotDerivePartialEq>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldNotDerivePartialEq),
            "::",
            stringify!(a)
        )
    );
}
impl Default for ShouldNotDerivePartialEq {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
