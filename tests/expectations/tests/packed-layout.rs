#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[test]
fn test_ComplexBitfieds() {
    assert_eq!(20usize, ::std::mem::size_of::<ComplexBitfieds>());
}
#[test]
fn test_ComplexBitfiedsPacked1() {
    assert_eq!(15usize, ::std::mem::size_of::<ComplexBitfiedsPacked1>());
}
#[test]
fn test_ComplexBitfiedsPacked2() {
    assert_eq!(16usize, ::std::mem::size_of::<ComplexBitfiedsPacked2>());
}

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
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AChar {
    pub c: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_AChar() {
    assert_eq!(
        ::std::mem::size_of::<AChar>(),
        1usize,
        concat!("Size of: ", stringify!(AChar))
    );
    assert_eq!(
        ::std::mem::align_of::<AChar>(),
        1usize,
        concat!("Alignment of ", stringify!(AChar))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AChar>())).c as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(AChar), "::", stringify!(c))
    );
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ACharPacked {
    pub c: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_ACharPacked() {
    assert_eq!(
        ::std::mem::size_of::<ACharPacked>(),
        1usize,
        concat!("Size of: ", stringify!(ACharPacked))
    );
    assert_eq!(
        ::std::mem::align_of::<ACharPacked>(),
        1usize,
        concat!("Alignment of ", stringify!(ACharPacked))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ACharPacked>())).c as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ACharPacked),
            "::",
            stringify!(c)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ACharShort {
    pub c: ::std::os::raw::c_char,
    pub s: ::std::os::raw::c_short,
}
#[test]
fn bindgen_test_layout_ACharShort() {
    assert_eq!(
        ::std::mem::size_of::<ACharShort>(),
        4usize,
        concat!("Size of: ", stringify!(ACharShort))
    );
    assert_eq!(
        ::std::mem::align_of::<ACharShort>(),
        2usize,
        concat!("Alignment of ", stringify!(ACharShort))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ACharShort>())).c as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ACharShort),
            "::",
            stringify!(c)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ACharShort>())).s as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(ACharShort),
            "::",
            stringify!(s)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ACharShortPacked1 {
    pub c: ::std::os::raw::c_char,
    pub s: ::std::os::raw::c_short,
}
#[test]
fn bindgen_test_layout_ACharShortPacked1() {
    assert_eq!(
        ::std::mem::size_of::<ACharShortPacked1>(),
        3usize,
        concat!("Size of: ", stringify!(ACharShortPacked1))
    );
    assert_eq!(
        ::std::mem::align_of::<ACharShortPacked1>(),
        1usize,
        concat!("Alignment of ", stringify!(ACharShortPacked1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ACharShortPacked1>())).c as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ACharShortPacked1),
            "::",
            stringify!(c)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ACharShortPacked1>())).s as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(ACharShortPacked1),
            "::",
            stringify!(s)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ACharShortPacked2 {
    pub c: ::std::os::raw::c_char,
    pub s: ::std::os::raw::c_short,
}
#[test]
fn bindgen_test_layout_ACharShortPacked2() {
    assert_eq!(
        ::std::mem::size_of::<ACharShortPacked2>(),
        4usize,
        concat!("Size of: ", stringify!(ACharShortPacked2))
    );
    assert_eq!(
        ::std::mem::align_of::<ACharShortPacked2>(),
        2usize,
        concat!("Alignment of ", stringify!(ACharShortPacked2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ACharShortPacked2>())).c as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ACharShortPacked2),
            "::",
            stringify!(c)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ACharShortPacked2>())).s as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(ACharShortPacked2),
            "::",
            stringify!(s)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ComplexBitfieds {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u16>,
    pub TFR: [::std::os::raw::c_uchar; 5usize],
    pub _bitfield_2: __BindgenBitfieldUnit<[u8; 4usize], u16>,
    pub SUM: ::std::os::raw::c_uint,
    pub RSV: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_ComplexBitfieds() {
    assert_eq!(
        ::std::mem::size_of::<ComplexBitfieds>(),
        20usize,
        concat!("Size of: ", stringify!(ComplexBitfieds))
    );
    assert_eq!(
        ::std::mem::align_of::<ComplexBitfieds>(),
        4usize,
        concat!("Alignment of ", stringify!(ComplexBitfieds))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ComplexBitfieds>())).TFR as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(ComplexBitfieds),
            "::",
            stringify!(TFR)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ComplexBitfieds>())).SUM as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ComplexBitfieds),
            "::",
            stringify!(SUM)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ComplexBitfieds>())).RSV as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ComplexBitfieds),
            "::",
            stringify!(RSV)
        )
    );
}
impl ComplexBitfieds {
    #[inline]
    pub fn TFS(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u16)
        }
    }
    #[inline]
    pub fn set_TFS(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn TDZ(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(4usize, 10u8) as u16)
        }
    }
    #[inline]
    pub fn set_TDZ(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn TDX(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(14usize, 2u8) as u16)
        }
    }
    #[inline]
    pub fn set_TDX(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        TFS: ::std::os::raw::c_ushort,
        TDZ: ::std::os::raw::c_ushort,
        TDX: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 2usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<
            [u8; 2usize],
            u16,
        > = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let TFS: u16 = unsafe { ::std::mem::transmute(TFS) };
            TFS as u64
        });
        __bindgen_bitfield_unit.set(4usize, 10u8, {
            let TDZ: u16 = unsafe { ::std::mem::transmute(TDZ) };
            TDZ as u64
        });
        __bindgen_bitfield_unit.set(14usize, 2u8, {
            let TDX: u16 = unsafe { ::std::mem::transmute(TDX) };
            TDX as u64
        });
        __bindgen_bitfield_unit
    }
    #[inline]
    pub fn TSS(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(0usize, 4u8) as u8)
        }
    }
    #[inline]
    pub fn set_TSS(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn TSI(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(4usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_TSI(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn _rb_(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(5usize, 2u8) as u8)
        }
    }
    #[inline]
    pub fn set__rb_(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(5usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn SDZ(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(16usize, 10u8) as u16)
        }
    }
    #[inline]
    pub fn set_SDZ(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_2.set(16usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn STR(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(26usize, 5u8) as u16)
        }
    }
    #[inline]
    pub fn set_STR(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_2.set(26usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn bRef(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(31usize, 1u8) as u16)
        }
    }
    #[inline]
    pub fn set_bRef(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_2.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_2(
        TSS: ::std::os::raw::c_uchar,
        TSI: ::std::os::raw::c_uchar,
        _rb_: ::std::os::raw::c_uchar,
        SDZ: ::std::os::raw::c_ushort,
        STR: ::std::os::raw::c_ushort,
        bRef: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<
            [u8; 4usize],
            u16,
        > = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let TSS: u8 = unsafe { ::std::mem::transmute(TSS) };
            TSS as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let TSI: u8 = unsafe { ::std::mem::transmute(TSI) };
            TSI as u64
        });
        __bindgen_bitfield_unit.set(5usize, 2u8, {
            let _rb_: u8 = unsafe { ::std::mem::transmute(_rb_) };
            _rb_ as u64
        });
        __bindgen_bitfield_unit.set(16usize, 10u8, {
            let SDZ: u16 = unsafe { ::std::mem::transmute(SDZ) };
            SDZ as u64
        });
        __bindgen_bitfield_unit.set(26usize, 5u8, {
            let STR: u16 = unsafe { ::std::mem::transmute(STR) };
            STR as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let bRef: u16 = unsafe { ::std::mem::transmute(bRef) };
            bRef as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ComplexBitfiedsPacked1 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
    pub TFR: [::std::os::raw::c_uchar; 5usize],
    pub _bitfield_2: __BindgenBitfieldUnit<[u8; 3usize], u8>,
    pub SUM: ::std::os::raw::c_uint,
    pub RSV: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_ComplexBitfiedsPacked1() {
    assert_eq!(
        ::std::mem::size_of::<ComplexBitfiedsPacked1>(),
        15usize,
        concat!("Size of: ", stringify!(ComplexBitfiedsPacked1))
    );
    assert_eq!(
        ::std::mem::align_of::<ComplexBitfiedsPacked1>(),
        1usize,
        concat!("Alignment of ", stringify!(ComplexBitfiedsPacked1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ComplexBitfiedsPacked1>())).TFR as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(ComplexBitfiedsPacked1),
            "::",
            stringify!(TFR)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ComplexBitfiedsPacked1>())).SUM as *const _
                as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(ComplexBitfiedsPacked1),
            "::",
            stringify!(SUM)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ComplexBitfiedsPacked1>())).RSV as *const _
                as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(ComplexBitfiedsPacked1),
            "::",
            stringify!(RSV)
        )
    );
}
impl ComplexBitfiedsPacked1 {
    #[inline]
    pub fn TFS(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u16)
        }
    }
    #[inline]
    pub fn set_TFS(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn TDZ(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(4usize, 10u8) as u16)
        }
    }
    #[inline]
    pub fn set_TDZ(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn TDX(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(14usize, 2u8) as u16)
        }
    }
    #[inline]
    pub fn set_TDX(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        TFS: ::std::os::raw::c_ushort,
        TDZ: ::std::os::raw::c_ushort,
        TDX: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 2usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<
            [u8; 2usize],
            u8,
        > = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let TFS: u16 = unsafe { ::std::mem::transmute(TFS) };
            TFS as u64
        });
        __bindgen_bitfield_unit.set(4usize, 10u8, {
            let TDZ: u16 = unsafe { ::std::mem::transmute(TDZ) };
            TDZ as u64
        });
        __bindgen_bitfield_unit.set(14usize, 2u8, {
            let TDX: u16 = unsafe { ::std::mem::transmute(TDX) };
            TDX as u64
        });
        __bindgen_bitfield_unit
    }
    #[inline]
    pub fn TSS(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(0usize, 4u8) as u8)
        }
    }
    #[inline]
    pub fn set_TSS(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn TSI(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(4usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_TSI(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn _rb_(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(5usize, 2u8) as u8)
        }
    }
    #[inline]
    pub fn set__rb_(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(5usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn SDZ(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(7usize, 10u8) as u16)
        }
    }
    #[inline]
    pub fn set_SDZ(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_2.set(7usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn STR(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(17usize, 5u8) as u16)
        }
    }
    #[inline]
    pub fn set_STR(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_2.set(17usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn bRef(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(22usize, 1u8) as u16)
        }
    }
    #[inline]
    pub fn set_bRef(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_2.set(22usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_2(
        TSS: ::std::os::raw::c_uchar,
        TSI: ::std::os::raw::c_uchar,
        _rb_: ::std::os::raw::c_uchar,
        SDZ: ::std::os::raw::c_ushort,
        STR: ::std::os::raw::c_ushort,
        bRef: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 3usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<
            [u8; 3usize],
            u8,
        > = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let TSS: u8 = unsafe { ::std::mem::transmute(TSS) };
            TSS as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let TSI: u8 = unsafe { ::std::mem::transmute(TSI) };
            TSI as u64
        });
        __bindgen_bitfield_unit.set(5usize, 2u8, {
            let _rb_: u8 = unsafe { ::std::mem::transmute(_rb_) };
            _rb_ as u64
        });
        __bindgen_bitfield_unit.set(7usize, 10u8, {
            let SDZ: u16 = unsafe { ::std::mem::transmute(SDZ) };
            SDZ as u64
        });
        __bindgen_bitfield_unit.set(17usize, 5u8, {
            let STR: u16 = unsafe { ::std::mem::transmute(STR) };
            STR as u64
        });
        __bindgen_bitfield_unit.set(22usize, 1u8, {
            let bRef: u16 = unsafe { ::std::mem::transmute(bRef) };
            bRef as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C, packed(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct ComplexBitfiedsPacked2 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u16>,
    pub TFR: [::std::os::raw::c_uchar; 5usize],
    pub _bitfield_2: __BindgenBitfieldUnit<[u8; 4usize], u16>,
    pub SUM: ::std::os::raw::c_uint,
    pub RSV: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_ComplexBitfiedsPacked2() {
    assert_eq!(
        ::std::mem::size_of::<ComplexBitfiedsPacked2>(),
        16usize,
        concat!("Size of: ", stringify!(ComplexBitfiedsPacked2))
    );
    assert_eq!(
        ::std::mem::align_of::<ComplexBitfiedsPacked2>(),
        2usize,
        concat!("Alignment of ", stringify!(ComplexBitfiedsPacked2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ComplexBitfiedsPacked2>())).TFR as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(ComplexBitfiedsPacked2),
            "::",
            stringify!(TFR)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ComplexBitfiedsPacked2>())).SUM as *const _
                as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(ComplexBitfiedsPacked2),
            "::",
            stringify!(SUM)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ComplexBitfiedsPacked2>())).RSV as *const _
                as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(ComplexBitfiedsPacked2),
            "::",
            stringify!(RSV)
        )
    );
}
impl ComplexBitfiedsPacked2 {
    #[inline]
    pub fn TFS(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u16)
        }
    }
    #[inline]
    pub fn set_TFS(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn TDZ(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(4usize, 10u8) as u16)
        }
    }
    #[inline]
    pub fn set_TDZ(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn TDX(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(14usize, 2u8) as u16)
        }
    }
    #[inline]
    pub fn set_TDX(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        TFS: ::std::os::raw::c_ushort,
        TDZ: ::std::os::raw::c_ushort,
        TDX: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 2usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<
            [u8; 2usize],
            u16,
        > = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let TFS: u16 = unsafe { ::std::mem::transmute(TFS) };
            TFS as u64
        });
        __bindgen_bitfield_unit.set(4usize, 10u8, {
            let TDZ: u16 = unsafe { ::std::mem::transmute(TDZ) };
            TDZ as u64
        });
        __bindgen_bitfield_unit.set(14usize, 2u8, {
            let TDX: u16 = unsafe { ::std::mem::transmute(TDX) };
            TDX as u64
        });
        __bindgen_bitfield_unit
    }
    #[inline]
    pub fn TSS(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(0usize, 4u8) as u8)
        }
    }
    #[inline]
    pub fn set_TSS(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn TSI(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(4usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_TSI(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn _rb_(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(5usize, 2u8) as u8)
        }
    }
    #[inline]
    pub fn set__rb_(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(5usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn SDZ(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(16usize, 10u8) as u16)
        }
    }
    #[inline]
    pub fn set_SDZ(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_2.set(16usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn STR(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(26usize, 5u8) as u16)
        }
    }
    #[inline]
    pub fn set_STR(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_2.set(26usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn bRef(&self) -> ::std::os::raw::c_ushort {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(31usize, 1u8) as u16)
        }
    }
    #[inline]
    pub fn set_bRef(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_2.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_2(
        TSS: ::std::os::raw::c_uchar,
        TSI: ::std::os::raw::c_uchar,
        _rb_: ::std::os::raw::c_uchar,
        SDZ: ::std::os::raw::c_ushort,
        STR: ::std::os::raw::c_ushort,
        bRef: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<
            [u8; 4usize],
            u16,
        > = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let TSS: u8 = unsafe { ::std::mem::transmute(TSS) };
            TSS as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let TSI: u8 = unsafe { ::std::mem::transmute(TSI) };
            TSI as u64
        });
        __bindgen_bitfield_unit.set(5usize, 2u8, {
            let _rb_: u8 = unsafe { ::std::mem::transmute(_rb_) };
            _rb_ as u64
        });
        __bindgen_bitfield_unit.set(16usize, 10u8, {
            let SDZ: u16 = unsafe { ::std::mem::transmute(SDZ) };
            SDZ as u64
        });
        __bindgen_bitfield_unit.set(26usize, 5u8, {
            let STR: u16 = unsafe { ::std::mem::transmute(STR) };
            STR as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let bRef: u16 = unsafe { ::std::mem::transmute(bRef) };
            bRef as u64
        });
        __bindgen_bitfield_unit
    }
}
