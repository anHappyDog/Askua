#ifndef __CSR_H__
#define __CSR_H__
#include <types.h>

#define DEFINE_R_CSR_FUNC(csr)                                                 \
  static inline size_t read_##csr(void) {                                      \
    size_t __val;                                                              \
    asm volatile("csrr %0, " #csr : "=r"(__val));                              \
    return __val;                                                              \
  }
#define DEFINE_W_CSR_FUNC(csr)                                                 \
  static inline void write_##csr(size_t __val) {                               \
    asm volatile("csrw " #csr ", %0" ::"r"(__val));                            \
  }

DEFINE_W_CSR_FUNC(stvec);
DEFINE_W_CSR_FUNC(sie);
DEFINE_R_CSR_FUNC(sie);
DEFINE_R_CSR_FUNC(sip);
DEFINE_W_CSR_FUNC(sstatus);
DEFINE_R_CSR_FUNC(sstatus);
DEFINE_R_CSR_FUNC(scause);
DEFINE_W_CSR_FUNC(time);
DEFINE_R_CSR_FUNC(time);
DEFINE_W_CSR_FUNC(stimecmp);
DEFINE_R_CSR_FUNC(stimecmp);
DEFINE_W_CSR_FUNC(satp);
DEFINE_R_CSR_FUNC(satp);
#endif // __CSR_H__