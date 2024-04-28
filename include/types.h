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

#define ROUNDDOWN(a, n) ((a) / (n) * (n))
#define ROUNDUP(a, n) (((a) + (n)-1) / (n) * (n))
#endif