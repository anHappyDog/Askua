#ifndef __MM_H__
#define __MM_H__
#include <mm/page.h>
#include <mm/pgtable.h>
#include <types.h>

#define RESTORE_MC_MC_TO_HS_STACK                                              \
  asm volatile("ld	ra,40(sp)\n"                                                \
               "ld  s0,32(sp)\n"                                               \
               "mv t0, %0\n"                                                   \
               "or ra,ra,t0\n"                                                 \
               "or sp,sp,t0\n"                                                 \
               "or s0,s0,t0\n"                                                 \
               "addi	sp,sp,48\n"                                               \
               "ret\n" ::"r"(VIRTUAL_KERNEL_BASE)                              \
               : "memory");

#define RESTORE_MM_MASTER_STACK                                                \
  asm volatile("mv t0, %0\n"                                                   \
               "ld	ra,40(sp)\n"                                                \
               "ld	s0,32(sp)\n"                                                \
               "or ra,ra,t0\n"                                                 \
               "or s0,s0,t0\n"                                                 \
               "addi	sp,sp,48\n"                                               \
               "ret\n" ::"r"(VIRTUAL_KERNEL_BASE)                              \
               : "memory");

#define RESTORE_MV_SC_TO_HS_STACK                                              \
  asm volatile("mv t0, %0\n"                                                   \
               "ld s0,8(sp)\n"                                                 \
               "or s0,s0,t0\n"                                                 \
               "or ra,ra,t0\n"                                                 \
               "addi	sp,sp,16\n"                                               \
               "ret\n" ::"r"(VIRTUAL_KERNEL_BASE)                              \
               : "memory");

#define RESTORE_MM_SLAVE_STACK                                                 \
  asm volatile("mv t0,%0\n"                                                    \
               "ld ra,24(sp)\n"                                                \
               "ld s0,16(sp)\n"                                                \
               "or ra,ra,t0\n"                                                 \
               "or s0,s0,t0\n"                                                 \
               "addi sp,sp,32\n"                                               \
               "ret\n" ::"r"(VIRTUAL_KERNEL_BASE)                              \
               : "memory");

#define TLB_FLUSH_ALL asm volatile("sfence.vma zero, zero\n" ::: "memory");

#define TLB_FLUSH(va, asid)                                                    \
  asm volatile("sfence.vma %0, %1\n" ::"r"(va), "r"(asid) : "memory");

// raw.c
size_t raw_heap_alloc(size_t size, size_t align);
void raw_heap_init(void);

void mm_master(size_t mem_base, size_t mem_size);
void mm_slave(void);
#endif // __MM_H__