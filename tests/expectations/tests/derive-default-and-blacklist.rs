#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub struct BlacklistMe(u8);

/// Because this type contains a blacklisted type, it should not derive
/// Default. Instead, we should emit a `mem::zeroed` implementation.
#[repr(C, packed)]
pub struct ShouldNotDeriveDefault {
    pub a: BlacklistMe,
}
#[test]
fn bindgen_test_layout_ShouldNotDeriveDefault() {
    assert_eq!(
        ::std::mem::size_of::<ShouldNotDeriveDefault>(),
        1usize,
        concat!("Size of: ", stringify!(ShouldNotDeriveDefault))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldNotDeriveDefault>(),
        1usize,
        concat!("Alignment of ", stringify!(ShouldNotDeriveDefault))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldNotDeriveDefault>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldNotDeriveDefault),
            "::",
            stringify!(a)
        )
    );
}
impl Default for ShouldNotDeriveDefault {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
