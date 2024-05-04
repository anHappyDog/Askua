#ifndef __TYPES_H__
#define __TYPES_H__
#include <stddef.h>
#include <stdint.h>
typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;

typedef uint64_t size_t;
typedef unsigned long ulong;

#define ROUNDDOWN(a, n) ((a) / (n) * (n))
#define ROUNDUP(a, n) (((a) + (n)-1) / (n) * (n))

#define __PREINIT__(x) __attribute__((section(".boot" #x)))
#define __NORETURN__ __attribute__((noreturn))
#define __ALWAYS_INLINE__ __attribute__((always_inline))
#define __PACKED__ __attribute__((packed))
#define __WEAK__ __attribute__((weak))
#define __SECTION__(x) __attribute__((section(#x)))
#define __ALIGN(x) __attribute__((aligned(x)))
#define __M_INTERRUPT__ __attribute__((interrupt("machine")))
#define __S_INTERRUPT__ __attribute__((interrupt("supervisor")))
#define __DEADLOOP__                                                           \
  while (1)                                                                    \
    ;

#endif