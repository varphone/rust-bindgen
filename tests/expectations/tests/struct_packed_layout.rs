#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Regular {
    pub f1: ::std::os::raw::c_char,
    pub f2: ::std::os::raw::c_short,
    pub f3: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Regular() {
    assert_eq!(
        ::std::mem::size_of::<Regular>(),
        8usize,
        concat!("Size of: ", stringify!(Regular))
    );
    assert_eq!(
        ::std::mem::align_of::<Regular>(),
        4usize,
        concat!("Alignment of ", stringify!(Regular))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Regular>())).f1 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Regular),
            "::",
            stringify!(f1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Regular>())).f2 as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(Regular),
            "::",
            stringify!(f2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Regular>())).f3 as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Regular),
            "::",
            stringify!(f3)
        )
    );
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Packed1WithAttr {
    pub f1: ::std::os::raw::c_char,
    pub f2: ::std::os::raw::c_short,
    pub f3: ::std::os::raw::c_int,
    pub f4: [::std::os::raw::c_char; 5usize],
    pub f5: Regular,
}
#[test]
fn bindgen_test_layout_Packed1WithAttr() {
    assert_eq!(
        ::std::mem::size_of::<Packed1WithAttr>(),
        20usize,
        concat!("Size of: ", stringify!(Packed1WithAttr))
    );
    assert_eq!(
        ::std::mem::align_of::<Packed1WithAttr>(),
        1usize,
        concat!("Alignment of ", stringify!(Packed1WithAttr))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed1WithAttr>())).f1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed1WithAttr),
            "::",
            stringify!(f1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed1WithAttr>())).f2 as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed1WithAttr),
            "::",
            stringify!(f2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed1WithAttr>())).f3 as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed1WithAttr),
            "::",
            stringify!(f3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed1WithAttr>())).f4 as *const _ as usize
        },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed1WithAttr),
            "::",
            stringify!(f4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed1WithAttr>())).f5 as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed1WithAttr),
            "::",
            stringify!(f5)
        )
    );
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Packed1WithPack {
    pub f1: ::std::os::raw::c_char,
    pub f2: ::std::os::raw::c_short,
    pub f3: ::std::os::raw::c_int,
    pub f4: [::std::os::raw::c_char; 5usize],
    pub f5: Regular,
}
#[test]
fn bindgen_test_layout_Packed1WithPack() {
    assert_eq!(
        ::std::mem::size_of::<Packed1WithPack>(),
        20usize,
        concat!("Size of: ", stringify!(Packed1WithPack))
    );
    assert_eq!(
        ::std::mem::align_of::<Packed1WithPack>(),
        1usize,
        concat!("Alignment of ", stringify!(Packed1WithPack))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed1WithPack>())).f1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed1WithPack),
            "::",
            stringify!(f1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed1WithPack>())).f2 as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed1WithPack),
            "::",
            stringify!(f2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed1WithPack>())).f3 as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed1WithPack),
            "::",
            stringify!(f3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed1WithPack>())).f4 as *const _ as usize
        },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed1WithPack),
            "::",
            stringify!(f4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed1WithPack>())).f5 as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed1WithPack),
            "::",
            stringify!(f5)
        )
    );
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct Packed2WithAttr {
    pub f1: ::std::os::raw::c_char,
    pub f2: ::std::os::raw::c_short,
    pub f3: ::std::os::raw::c_int,
    pub f4: [::std::os::raw::c_char; 5usize],
    pub f5: Regular,
    pub __bindgen_padding_0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_Packed2WithAttr() {
    assert_eq!(
        ::std::mem::size_of::<Packed2WithAttr>(),
        22usize,
        concat!("Size of: ", stringify!(Packed2WithAttr))
    );
    assert_eq!(
        ::std::mem::align_of::<Packed2WithAttr>(),
        2usize,
        concat!("Alignment of ", stringify!(Packed2WithAttr))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed2WithAttr>())).f1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed2WithAttr),
            "::",
            stringify!(f1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed2WithAttr>())).f2 as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed2WithAttr),
            "::",
            stringify!(f2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed2WithAttr>())).f3 as *const _ as usize
        },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed2WithAttr),
            "::",
            stringify!(f3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed2WithAttr>())).f4 as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed2WithAttr),
            "::",
            stringify!(f4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed2WithAttr>())).f5 as *const _ as usize
        },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed2WithAttr),
            "::",
            stringify!(f5)
        )
    );
}
#[repr(C, packed(2))]
#[derive(Copy, Clone)]
pub struct Packed2WithPack {
    pub f1: ::std::os::raw::c_char,
    pub f2: ::std::os::raw::c_short,
    pub f3: ::std::os::raw::c_int,
    pub f4: [::std::os::raw::c_char; 5usize],
    pub f5: Regular,
    pub __bindgen_padding_0: [u8; 2usize],
}
#[test]
fn bindgen_test_layout_Packed2WithPack() {
    assert_eq!(
        ::std::mem::size_of::<Packed2WithPack>(),
        22usize,
        concat!("Size of: ", stringify!(Packed2WithPack))
    );
    assert_eq!(
        ::std::mem::align_of::<Packed2WithPack>(),
        2usize,
        concat!("Alignment of ", stringify!(Packed2WithPack))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed2WithPack>())).f1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed2WithPack),
            "::",
            stringify!(f1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed2WithPack>())).f2 as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed2WithPack),
            "::",
            stringify!(f2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed2WithPack>())).f3 as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed2WithPack),
            "::",
            stringify!(f3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed2WithPack>())).f4 as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed2WithPack),
            "::",
            stringify!(f4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed2WithPack>())).f5 as *const _ as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed2WithPack),
            "::",
            stringify!(f5)
        )
    );
}
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct Packed4WithAttr {
    pub f1: ::std::os::raw::c_char,
    pub f2: ::std::os::raw::c_short,
    pub f3: ::std::os::raw::c_int,
    pub f4: [::std::os::raw::c_char; 5usize],
    pub f5: Regular,
    pub __bindgen_padding_0: [u8; 8usize],
}
#[test]
fn bindgen_test_layout_Packed4WithAttr() {
    assert_eq!(
        ::std::mem::size_of::<Packed4WithAttr>(),
        28usize,
        concat!("Size of: ", stringify!(Packed4WithAttr))
    );
    assert_eq!(
        ::std::mem::align_of::<Packed4WithAttr>(),
        4usize,
        concat!("Alignment of ", stringify!(Packed4WithAttr))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed4WithAttr>())).f1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed4WithAttr),
            "::",
            stringify!(f1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed4WithAttr>())).f2 as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed4WithAttr),
            "::",
            stringify!(f2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed4WithAttr>())).f3 as *const _ as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed4WithAttr),
            "::",
            stringify!(f3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed4WithAttr>())).f4 as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed4WithAttr),
            "::",
            stringify!(f4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed4WithAttr>())).f5 as *const _ as usize
        },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed4WithAttr),
            "::",
            stringify!(f5)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Packed4WithPack {
    pub f1: ::std::os::raw::c_char,
    pub f2: ::std::os::raw::c_short,
    pub f3: ::std::os::raw::c_int,
    pub f4: [::std::os::raw::c_char; 5usize],
    pub f5: Regular,
}
#[test]
fn bindgen_test_layout_Packed4WithPack() {
    assert_eq!(
        ::std::mem::size_of::<Packed4WithPack>(),
        24usize,
        concat!("Size of: ", stringify!(Packed4WithPack))
    );
    assert_eq!(
        ::std::mem::align_of::<Packed4WithPack>(),
        4usize,
        concat!("Alignment of ", stringify!(Packed4WithPack))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed4WithPack>())).f1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed4WithPack),
            "::",
            stringify!(f1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed4WithPack>())).f2 as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed4WithPack),
            "::",
            stringify!(f2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed4WithPack>())).f3 as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed4WithPack),
            "::",
            stringify!(f3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed4WithPack>())).f4 as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed4WithPack),
            "::",
            stringify!(f4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Packed4WithPack>())).f5 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Packed4WithPack),
            "::",
            stringify!(f5)
        )
    );
}
