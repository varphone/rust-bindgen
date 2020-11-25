#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub struct BlacklistMe(u8);

/// Because this type contains a blacklisted type, it should not derive Debug.
#[repr(C, packed)]
pub struct ShouldManuallyImplDebug {
    pub a: BlacklistMe,
}
#[test]
fn bindgen_test_layout_ShouldManuallyImplDebug() {
    assert_eq!(
        ::std::mem::size_of::<ShouldManuallyImplDebug>(),
        1usize,
        concat!("Size of: ", stringify!(ShouldManuallyImplDebug))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldManuallyImplDebug>(),
        1usize,
        concat!("Alignment of ", stringify!(ShouldManuallyImplDebug))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldManuallyImplDebug>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldManuallyImplDebug),
            "::",
            stringify!(a)
        )
    );
}
impl Default for ShouldManuallyImplDebug {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ShouldManuallyImplDebug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "ShouldManuallyImplDebug {{  }}")
    }
}
