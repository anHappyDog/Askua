#ifndef __MMU_H__
#define __MMU_H__
#include <mm/page.h>

#define MEM_BASE 0x80000000UL
#define MEM_SIZE 0x10000000UL

#define VIRTUAL_FS_BASE 0xFFFFF80000000000UL
#define VIRTUAL_PAGING_BASE 0xfffffff000000000UL
#define VIRTUAL_KERNEL_BASE 0xFFFFFFF000000000UL

#define PREINIT_BASE 0x80200000UL
#define PRESTACK_BASE 0x80202000UL
#define PRESTACK_SZ 0x4000UL
#define PRESTACK_TOP (PRESTACK_BASE + PRESTACK_SZ)
#define PREHEAP_BASE (PRESTACK_BASE + PRESTACK_SZ)
#define PREHEAP_SZ 0x10000UL

#define PHYSICAL_STACK_BASE (PREHEAP_BASE + PREHEAP_SZ)
#define STACK_SZ 0x40000UL
#define STACK_SZ_PER_CORE 0x10000UL
#define RAWHEAP_BASE (PHYSICAL_STACK_BASE + STACK_SZ)
#define RAWHEAP_SZ 0x800000UL

#define PHYSICAL_KERNEL_BASE (RAWHEAP_BASE + RAWHEAP_SZ)
#define PHYSICAL_JUMPER_BASE PHYSICAL_KERNEL_BASE
#define PHYSICAL_JUMPER_DATA_BASE (PHYSICAL_JUMPER_BASE + PAGE_SIZE)
#define PHYSICAL_TEXT_BASE (PHYSICAL_JUMPER_DATA_BASE + PAGE_SIZE)

#define __TO_JUMPER__(hartid, dtbptr)                                          \
  ({                                                                           \
    void (*__jumper)(size_t, size_t) = (void (*)(size_t, size_t))(             \
        PHYSICAL_JUMPER_BASE | VIRTUAL_KERNEL_BASE);                           \
    __jumper((size_t)hartid, (size_t)dtbptr);                                  \
  });

#define __JUMPER_RESTORE_STACK(hartid)                                         \
  ({                                                                           \
    asm volatile("mv sp, %0\n"                                                 \
                 "mv t0, %1\n"                                                 \
                 "mul t0,t0,%2\n"                                              \
                 "sub sp,sp,t0\n" ::"r"(                                       \
                     (VIRTUAL_KERNEL_BASE | PHYSICAL_STACK_BASE) + STACK_SZ),  \
                 "r"(STACK_SZ_PER_CORE), "r"((size_t)hartid));                 \
  });

#define __SLAVE_ENABLE_MMU__(pre_pgd)                                          \
  ({                                                                           \
    write_pre_satp((((size_t)pre_pgd & ~VIRTUAL_KERNEL_BASE) >> PAGE_SHIFT) |  \
                   SATP_SV39_MODE);                                            \
  });

#define __SLAVE_JUMP_TO_INIT__                                                 \
  ({                                                                           \
    void (*__init)(size_t, size_t) =                                           \
        (void (*)(size_t, size_t))(PHYSICAL_TEXT_BASE | VIRTUAL_KERNEL_BASE);  \
    __init((size_t)hartid, (size_t)dtbptr);                                    \
  });

#endif //__MMU_H__