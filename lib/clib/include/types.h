#ifndef __TYPES_H__
#define __TYPES_H__
#include <stddef.h>
#include <stdbool.h>
typedef char int8_t;
typedef short int16_t;
typedef int int32_t;
typedef long int64_t;
typedef __int128 int128_t;
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long uint64_t;
typedef unsigned __int128 uint128_t;

typedef unsigned char uchar;
typedef unsigned short ushort;
typedef unsigned int uint;
typedef unsigned long ulong;

typedef int64_t ssize_t;
typedef uint64_t size_t;
typedef int64_t off_t;
typedef uint64_t ino_t;
typedef uint64_t loff_t;
typedef uint64_t blkcnt_t;
typedef uint32_t blksize_t;
typedef unsigned int umode_t;
typedef unsigned int uid_t;
typedef unsigned int gid_t;
typedef unsigned int kuid_t;
typedef unsigned int kgid_t;
typedef unsigned long dev_t;
typedef unsigned int nlink_t;
typedef unsigned int mode_t;

#define ROUND_UP(a, n) (((a) + (n) - 1) & ~((n) - 1))
#define ROUND_DOWN(a, n) ((a) & ~((n) - 1))
#define MIN(_a, _b)                                                                                \
	({                                                                                         \
		typeof(_a) __a = (_a);                                                             \
		typeof(_b) __b = (_b);                                                             \
		__a < __b ? __a : __b;                                                             \
	})
#define MAX(_a, _b)                                                                                \
	({                                                                                         \
		typeof(_a) __a = (_a);                                                             \
		typeof(_b) __b = (_b);                                                             \
		__a > __b ? __a : __b;                                                             \
	})

#endif
