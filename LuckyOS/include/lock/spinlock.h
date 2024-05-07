#ifndef __SPINLOCK_H_
#define __SPINLOCK_H_
#include <asm/barrier.h>
#include <atomic.h>
#include <trap.h>
typedef struct {
  atomic_t lock;
} spinlock_t;

#define SPIN_INIT                                                              \
  { 0 }

static inline int spin_trylock(spinlock_t *lock) {
  int tmp = 1, busy = 0;
  asm volatile("   amoswap.w %0,%2,%1\n"
               : "=r"(busy), "+A"(lock->lock)
               : "r"(tmp)
               : "memory", "gp");
  RISCV_FENCE(r, rw);
  return !busy;
}

static inline int spin_try_unlock(spinlock_t *lock) {
  volatile atomic_t *lock_ptr = &(lock->lock);
  atomic_write(lock_ptr, 0);
  return 1;
}

static inline int spin_is_locked(spinlock_t *lock) {
  const atomic_t v = atomic_read(&(lock->lock));

  return v != 0;
}

static inline void spin_lock(spinlock_t *lock) {
  while (1) {
    if (spin_is_locked(lock)) {
      continue;
    }
    if (spin_trylock(lock)) {
      break;
    }
  }
}

static inline void spin_unlock(spinlock_t *lock) {
  while (1) {
    if (spin_try_unlock(lock)) {
      break;
    }
  }
}

/*
spin_lock_irq() = spin_lock() + local_irq_disable()
spin_unlock_irq = spin_unlock() + local_irq_enable()
spin_lock_irqsave = spin_lock() + local_irq_save()
spin_lock_irqrestore() = spin_unlock() + local_irq_restore()
spin_lock_bh() = spin_lock() + local_bh_disable()
spin_nlock_bh() = spin_unlock() + local_bh_enable()
*/

#endif // __SPINLOCK_H_