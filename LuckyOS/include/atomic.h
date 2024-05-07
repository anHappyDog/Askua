#ifndef __ATOMIC_H_
#define __ATOMIC_H_
#include <asm/barrier.h>
#include <types.h>
typedef uint32_t atomic_t;

static inline atomic_t atomic_read(volatile atomic_t *ptr) {
  atomic_t val;
  val = *(volatile atomic_t *)ptr;
  RISCV_FENCE(rw, w);

  return val;
}

static inline void atomic_write(volatile atomic_t *ptr, atomic_t __val) {
  RISCV_FENCE(r, rw);
  *(volatile atomic_t *)ptr = __val;
}

#endif // __ATOMIC_H_