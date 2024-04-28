#include <dev/plic.h>
#include <printk.h>
#include <smp.h>

void plic_init(size_t base, size_t size)
{
	size_t hartid = SMP_GET_HARTID();
	*(volatile uint32_t *)(base + VIRTIO_MMIO_INT * 4) = 2;
	*(volatile uint32_t *)(base + GOLD_FISH_RTC_INT * 4) = 1;

	*(volatile uint32_t *)(base + SIFIVE_INT_ENABLE_BASE +
						   SIFIVE_ENABLE_CONTEXT_BLOCK_SIZE * (2 * hartid + 1) +
						   (VIRTIO_MMIO_INT / 32) * 4) |= 1 << (VIRTIO_MMIO_INT % 32);
	*(volatile uint32_t *)(base + SIFIVE_INT_PRI_THRESHOLD_BASE + 0x1000 * (2 * hartid + 1)) = 0;

	printk("plic: init finished\n");
}

uint32_t plic_claim(size_t base, size_t context)
{
	return *(volatile uint32_t *)(base + SIFIVE_INT_PRI_CLAIM_BASE + 0x1000 * context);
}

void plic_complete(size_t base, size_t context, uint32_t irq)
{
	*(volatile uint32_t *)(base + SIFIVE_INT_PRI_CLAIM_BASE + 0x1000 * context) = irq;
}