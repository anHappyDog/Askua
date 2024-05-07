#ifndef __PLIC_H__
#define __PLIC_H__
#include <types.h>
//! hardcore the base address and size of the PLIC **temporarily**
//! virt's dts shows that it only supports the 0x9 and 0xb interrupts.
//! which is the VirtIO's interrupt and the RTC's interrupt.
#define SIFIVE_BASE_ADDR 0x0C000000UL
#define SIFIVE_INT_PRI_BASE 0x0
#define SIFIVE_INT_PENDING_BASE 0x1000
#define SIFIVE_INT_ENABLE_BASE 0x2000
#define SIFIVE_INT_PRI_THRESHOLD_BASE 0x200000
#define SIFIVE_INT_PRI_CLAIM_BASE 0x200004

#define SIFIVE_INT_NUM 1024

#define SIFIVE_ENABLE_CONTEXT_BLOCK_SIZE 0x80
#define SIFIVE_BASE_SIZE 0x600000

#define VIRTIO_MMIO_INT 0x8
#define GOLD_FISH_RTC_INT 0xb

void plic_init(size_t base, size_t size);
u32 plic_claim(size_t context);
void plic_complete(size_t context, u32 irq);

#endif // __PLIC_H__