struct WithBitfield {
    unsigned : 7;
    unsigned a;
};

struct WithBitfieldAndAttrPacked {
    unsigned : 7;
    unsigned a;
} __attribute__((packed));

#pragma pack(1)
struct WithBitfieldAndPacked {
    unsigned : 7;
    unsigned a;
};
