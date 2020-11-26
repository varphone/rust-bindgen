// bindgen-flags: --no-derive-debug --no-derive-default --whitelist-type "Packed.*"

// For stuff.
struct Regular {
    char f1;
    short f2;
    int f3;
};

// Packing with `1` byte alignment by `__attribute__((aligned(1)))`.
struct Packed1WithAttr
{
    char f1 __attribute__((aligned(1)));
    short f2;
    int f3;
    char f4[5] __attribute__((aligned(1)));
    struct Regular f5;
} __attribute__((packed));

// Packing with `1` byte alignment by `#pragma pack(push,1))`.
#pragma pack(push,1)
struct Packed1WithPack
{
    char f1;
    short f2;
    int f3;
    char f4[5];
    struct Regular f5;
};
#pragma pack(pop)

// Packing with `2` bytes alignment by `__attribute__((aligned(2)))`.
// Expected `sizeof(Packed2WithAttr) == 22`.
struct Packed2WithAttr
{
    char f1 __attribute__((aligned(2)));
    short f2 __attribute__((aligned(2)));
    int f3 __attribute__((aligned(2)));
    char f4[5] __attribute__((aligned(2)));
    struct Regular f5 __attribute__((aligned(2)));
} __attribute__((packed, aligned(2)));

// Packing with `2` bytes alignment by `#pragma pack(push,2))`.
// Expected `sizeof(Packed2WithPack) == 22`.
#pragma pack(push,2)
struct Packed2WithPack
{
    char f1;
    short f2;
    int f3;
    char f4[5];
    struct Regular f5;
};
#pragma pack(pop)

// Packing with `4` bytes alignment by `__attribute__((aligned(4)))`.
struct Packed4WithAttr
{
    char f1 __attribute__((aligned(4)));
    short f2 __attribute__((aligned(4)));
    int f3 __attribute__((aligned(4)));
    char f4[5] __attribute__((aligned(4)));
    struct Regular f5 __attribute__((aligned(4)));
} __attribute__((packed, aligned(4)));

// Packing with `4` bytes alignment by `#pragma pack(push,4))`.
#pragma pack(push,4)
struct Packed4WithPack
{
    char f1;
    short f2;
    int f3;
    char f4[5];
    struct Regular f5;
};
#pragma pack(pop)

#ifdef TEST
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#ifndef TEST
#include "tests/headers/struct_packed_layout.h"
#endif

#define offsetof(type, member) __builtin_offsetof(type, member)

#define print_header() printf("            type size  f1  f2  f3  f4  f5\n")

#define print_type(T)                                                          \
  printf("%16s %4lu %3lu %3lu %3lu %3lu %3lu\n", __STRING(T), sizeof(T),       \
         offsetof(T, f1), offsetof(T, f2), offsetof(T, f3), offsetof(T, f4),   \
         offsetof(T, f5))

int main(int argc, char **arv) {
  print_header();
  print_type(Packed1WithAttr);
  print_type(Packed1WithPack);
  print_type(Packed2WithAttr);
  print_type(Packed2WithPack);
  print_type(Packed4WithAttr);
  print_type(Packed4WithPack);
  return 0;
}
#endif
