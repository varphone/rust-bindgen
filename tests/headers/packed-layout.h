struct AChar {
    char c;
};

#pragma pack(push,1)
struct ACharPacked {
    char c;
};
#pragma pack(pop)

struct ACharShort {
    char c;
    short s;
};

#pragma pack(push,1)
struct ACharShortPacked1 {
    char c;
    short s;
};
#pragma pack(pop)

#pragma pack(push,2)
struct ACharShortPacked2 {
    char c;
    short s;
};
#pragma pack(pop)

// Expected 20bytes
// bindgen-flags: --raw-line '#[test] fn test_ComplexBitfieds() { assert_eq!(20usize, ::std::mem::size_of::<ComplexBitfieds>()); }'
struct ComplexBitfieds {
    unsigned short TFS : 4,  TDZ : 10, TDX : 2;
    unsigned char  TFR[5],   TSS : 4,  TSI : 1, _rb_ : 2;
    unsigned short SDZ : 10, STR : 5,  bRef : 1;
    unsigned int SUM;
    unsigned char RSV;
};

// Expected 15bytes
// bindgen-flags: --raw-line '#[test] fn test_ComplexBitfiedsPacked1() { assert_eq!(15usize, ::std::mem::size_of::<ComplexBitfiedsPacked1>()); }'
#pragma pack(push,1)
struct ComplexBitfiedsPacked1 {
    unsigned short TFS : 4,  TDZ : 10, TDX : 2;
    unsigned char  TFR[5],   TSS : 4,  TSI : 1, _rb_ : 2;
    unsigned short SDZ : 10, STR : 5,  bRef : 1;
    unsigned int SUM;
    unsigned char RSV;
};
#pragma pack(pop)

// Expected 16bytes
// bindgen-flags: --raw-line '#[test] fn test_ComplexBitfiedsPacked2() { assert_eq!(16usize, ::std::mem::size_of::<ComplexBitfiedsPacked2>()); }'
#pragma pack(push,2)
struct ComplexBitfiedsPacked2 {
    unsigned short TFS : 4,  TDZ : 10, TDX : 2;
    unsigned char  TFR[5],   TSS : 4,  TSI : 1, _rb_ : 2;
    unsigned short SDZ : 10, STR : 5,  bRef : 1;
    unsigned int SUM;
    unsigned char RSV;
};
#pragma pack(pop)
