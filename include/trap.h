#ifndef __TRAP_H__
#define __TRAP_H__
#include <types.h>

#define S_SOFTWARE_INT 1
#define M_SOFTWARE_INT 3
#define S_TIMER_INT 5
#define M_TIMER_INT 7
#define M_EXTERN_INT 11

#define INSTR_ADDR_MISALIGNED 0
#define INSTR_ACCESS_FAULT 1
#define ILLEGAL_INSTRUCTION 2
#define BREAKPOINT 3
#define LOAD_ADDR_MISALIGNED 4
#define LOAD_ACCESS_FAULT 5
#define STORE_ADDR_MISALIGNED 6
#define STORE_ACCESS_FAULT 7
#define ENV_CALL_FROM_U_MODE 8
#define ENV_CALL_FROM_S_MODE 9
#define ENV_CALL_FROM_M_MODE 11
#define INSTR_PAGE_FAULT 12
#define LOAD_PAGE_FAULT 13
#define STORE_PAGE_FAULT 15

#define STVEC_VECTOR 0X1
#define STVEC_DIRECT 0x0

#define SIE_SEIE 0x200
#define SIE_STIE 0x020
#define SIE_SSIE 0x002

#define SSTATUS_SIE 0x02

void trap_init(void);

inline void enable_irq();
inline u8 is_irq_enabled();
inline void disable_irq();
#endif // __TRAP_H__