#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct UnknownUnits {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_UnknownUnits() {
    assert_eq!(
        ::std::mem::size_of::<UnknownUnits>(),
        1usize,
        concat!("Size of: ", stringify!(UnknownUnits))
    );
    assert_eq!(
        ::std::mem::align_of::<UnknownUnits>(),
        1usize,
        concat!("Alignment of ", stringify!(UnknownUnits))
    );
}
pub type Float = f32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PointTyped<F> {
    pub x: F,
    pub y: F,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<F>>,
}
impl<F> Default for PointTyped<F> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type IntPoint = PointTyped<f32>;
