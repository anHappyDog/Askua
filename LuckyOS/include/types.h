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
#define __JUMPER_DATA__ __SECTION__(.jumper.data)
#define __PREINIT_DATA__ __PREINIT__(.data)
#define __TEXT_INIT__ __SECTION__(.text.init)
#define __JUMPER__ __SECTION__(.text.jumper)
#define __PREINIT_START__ __PREINIT__(._start)
#define __JUMPER_KMMAP__ __SECTION__(.jumper.kmmap)
#define __JUMP_TO_INIT__(x, y)                                                 \
  ({                                                                           \
    static int __JUMPER_DATA__ __master = 0;                                   \
    if (__master == 0) {                                                       \
      __master = 1;                                                            \
      {x};                                                                     \
    }                                                                          \
    {y};                                                                       \
  })

#define __DEADLOOP__                                                           \
  while (1)                                                                    \
    ;

#define __LOGO__                                                               \
  printk(                                                                      \
      " ________  ________  ___  __    ___  ___  ________\n"                   \
      "|\\   __  \\|\\   ____\\|\\  \\|\\  \\ |\\  \\|\\  \\|\\   __  \\\n"    \
      "\\ \\  \\|\\  \\ \\  \\___|\\ \\  \\/  /|\\ \\  \\\\\\  \\ \\  \\|\\  " \
      "\\\n"                                                                   \
      " \\ \\   __  \\ \\_____  \\ \\   ___  \\ \\  \\\\\\  \\ \\   __  \\\n"  \
      "  \\ \\  \\ \\  \\|____|\\  \\ \\  \\\\ \\  \\ \\  \\\\\\  \\ \\  \\ "  \
      "\\  \\\n"                                                               \
      "   \\ \\__\\ \\__\\____\\_\\  \\ \\__\\\\ \\__\\ \\_______\\ \\__\\ "   \
      "\\__\\\n"                                                               \
      "    \\|__|\\|__|\\_________\\|__| \\|__|\\|_______|\\|__|\\|__|\n"      \
      "             \\|_________|\n");

#endif