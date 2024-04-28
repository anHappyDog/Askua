#ifndef __SMP_H__
#define __SMP_H__
#include <types.h>
#define CORE 4
struct cpu_info {
    /* init by the main core */
    size_t stack_size;
    size_t stack_top;
    size_t hart_id;
    uint8_t master;
};

// get the hart from gp,the nude program compiler doesn't use
// it so we can use it to store the hartid instead of using 
// ecall.
#define SMP_GET_HARTID() ({ \
    size_t __val; \
    asm volatile ("mv %0,gp" : "=r" (__val)); \
    __val; \
})


#endif 