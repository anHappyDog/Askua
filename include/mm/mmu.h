#ifndef __MMU_H__
#define __MMU_H__
#include <mm/page.h>

#define VIRTUAL_FS_BASE 0xFFFFF80000000000UL
#define VIRTUAL_PAGING_BASE 0xffff800000000000UL
#define VIRTUAL_KERNEL_BASE 0xFFFFFFF000000000UL

#define PREINIT_BASE 0x80200000UL
#define PRESTACK_BASE 0x80202000UL
#define PRESTACK_SZ 0x4000UL
#define PREHEAP_BASE (PRESTACK_BASE + PRESTACK_SZ)
#define PREHEAP_SZ 0x10000UL

#define PHYSICAL_STACK_BASE (PREHEAP_BASE + PREHEAP_SZ)
#define STACK_SZ 0x40000UL
#define STACK_SZ_PER_CORE 0x10000UL
#define RAWHEAP_BASE (PHYSICAL_STACK_BASE + STACK_SZ)
#define RAWHEAP_SZ 0x80000UL

#define PHYSICAL_KERNEL_BASE (RAWHEAP_BASE + RAWHEAP_SZ)
#define PHYSICAL_JUMPER_BASE PHYSICAL_KERNEL_BASE
#define PHYSICAL_JUMPER_DATA_BASE (PHYSICAL_JUMPER_BASE + PAGE_SIZE)

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
                 "sub sp,sp,t0\n"                                              \
         ::"r"((VIRTUAL_KERNEL_BASE | PHYSICAL_STACK_BASE) + STACK_SZ),        \
                 "r"(STACK_SZ_PER_CORE), "r"((size_t)hartid));                 \
  });

#endif //__MMU_H__