#include <dev/plic.h>
#include <mm/mm.h>
#include <printk.h>
#include <smp.h>

static void _plic_init(size_t base, size_t size);
static u32 _plic_claim(size_t base, size_t context);
static void _plic_complete(size_t base, size_t context, u32 irq);

struct plic_operations_struct {
  void (*init)(size_t base, size_t size);
  u32 (*claim)(size_t base, size_t context);
  void (*complete)(size_t base, size_t context, u32 irq);
};



static struct plic_operations_struct plic_ops = {
    .init = _plic_init,
    .claim = _plic_claim,
    .complete = _plic_complete,
};

struct plic {
  size_t pm_base;
  size_t pm_size;
  struct plic_operations_struct *ops;
};

static struct plic plic = {
    .pm_base = SIFIVE_BASE_ADDR,
    .pm_size = SIFIVE_BASE_SIZE,
    .ops = &plic_ops,
};

static u32 _plic_claim(size_t base, size_t context) {
  return *(volatile u32 *)(base + VIRTUAL_KERNEL_BASE +
                           SIFIVE_INT_PRI_CLAIM_BASE + 0x1000 * context);
}

static void _plic_complete(size_t base, size_t context, u32 irq) {
  *(volatile u32 *)(base + VIRTUAL_KERNEL_BASE + SIFIVE_INT_PRI_CLAIM_BASE +
                    0x1000 * context) = irq;
}

void plic_init(size_t base, size_t size) { plic.ops->init(plic.pm_base | VIRTUAL_KERNEL_BASE, plic.pm_size); }

inline void _plic_init(size_t base, size_t size) {
  size_t hartid = SMP_GET_HARTID();
  *(volatile u32 *)(base + VIRTIO_MMIO_INT * 4) = 2;
  *(volatile u32 *)(base + GOLD_FISH_RTC_INT * 4) = 1;

  *(volatile u32 *)(base + SIFIVE_INT_ENABLE_BASE +
                    SIFIVE_ENABLE_CONTEXT_BLOCK_SIZE * (2 * hartid + 1) +
                    (VIRTIO_MMIO_INT / 32) * 4) |= 1 << (VIRTIO_MMIO_INT % 32);
  *(volatile u32 *)(base + SIFIVE_INT_PRI_THRESHOLD_BASE +
                    0x1000 * (2 * hartid + 1)) = 0;

  printk("plic: init finished\n");
}

u32 plic_claim(size_t context) {
  return plic.ops->claim(plic.pm_base, context);
}

void plic_complete(size_t context, u32 irq) {
  plic.ops->complete(plic.pm_base, context, irq);
}