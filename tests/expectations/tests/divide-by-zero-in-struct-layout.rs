#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WithBitfield {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 0usize], u8>,
    pub __bindgen_padding_0: u32,
    pub a: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_WithBitfield() {
    assert_eq!(
        ::std::mem::size_of::<WithBitfield>(),
        8usize,
        concat!("Size of: ", stringify!(WithBitfield))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBitfield>(),
        4usize,
        concat!("Alignment of ", stringify!(WithBitfield))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBitfield>())).a as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBitfield),
            "::",
            stringify!(a)
        )
    );
}
impl WithBitfield {
    #[inline]
    pub fn new_bitfield_1() -> __BindgenBitfieldUnit<[u8; 0usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<
            [u8; 0usize],
            u8,
        > = Default::default();
        __bindgen_bitfield_unit
    }
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WithBitfieldAndAttrPacked {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub a: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_WithBitfieldAndAttrPacked() {
    assert_eq!(
        ::std::mem::size_of::<WithBitfieldAndAttrPacked>(),
        5usize,
        concat!("Size of: ", stringify!(WithBitfieldAndAttrPacked))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBitfieldAndAttrPacked>(),
        1usize,
        concat!("Alignment of ", stringify!(WithBitfieldAndAttrPacked))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBitfieldAndAttrPacked>())).a as *const _
                as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBitfieldAndAttrPacked),
            "::",
            stringify!(a)
        )
    );
}
impl WithBitfieldAndAttrPacked {
    #[inline]
    pub fn new_bitfield_1() -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<
            [u8; 1usize],
            u8,
        > = Default::default();
        __bindgen_bitfield_unit
    }
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WithBitfieldAndPacked {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub a: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_WithBitfieldAndPacked() {
    assert_eq!(
        ::std::mem::size_of::<WithBitfieldAndPacked>(),
        5usize,
        concat!("Size of: ", stringify!(WithBitfieldAndPacked))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBitfieldAndPacked>(),
        1usize,
        concat!("Alignment of ", stringify!(WithBitfieldAndPacked))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBitfieldAndPacked>())).a as *const _
                as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBitfieldAndPacked),
            "::",
            stringify!(a)
        )
    );
}
impl WithBitfieldAndPacked {
    #[inline]
    pub fn new_bitfield_1() -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<
            [u8; 1usize],
            u8,
        > = Default::default();
        __bindgen_bitfield_unit
    }
}
