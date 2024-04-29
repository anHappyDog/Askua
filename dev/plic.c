#include <dev/plic.h>
#include <mm/mm.h>
#include <printk.h>
#include <smp.h>

static void _plic_init(size_t base, size_t size);
static u32 plic_claim_um(size_t base, size_t context);
static void plic_complete_um(size_t base, size_t context, u32 irq);
static u32 plic_claim_mm(size_t base, size_t context);
static void plic_complete_mm(size_t base, size_t context, u32 irq);

struct plic_operations_struct {
  void (*init)(size_t base, size_t size);
  u32 (*claim)(size_t base, size_t context);
  void (*complete)(size_t base, size_t context, u32 irq);
};

static struct plic_operations_struct plic_um_ops = {
    .init = _plic_init,
    .claim = plic_claim_um,
    .complete = plic_complete_um,
};

static struct plic_operations_struct plic_mm_ops = {
    .init = _plic_init,
    .claim = plic_claim_mm,
    .complete = plic_complete_mm,
};

struct plic {
  size_t pm_base;
  size_t pm_size;
  struct plic_operations_struct *ops;
};

static struct plic plic = {
    .pm_base = 0,
    .pm_size = 0,
    .ops = &plic_um_ops,
};

static u32 plic_claim_um(size_t base, size_t context) {
  return *(volatile u32 *)(base + SIFIVE_INT_PRI_CLAIM_BASE + 0x1000 * context);
}

static void plic_complete_um(size_t base, size_t context, u32 irq) {
  *(volatile u32 *)(base + SIFIVE_INT_PRI_CLAIM_BASE + 0x1000 * context) = irq;
}

static u32 plic_claim_mm(size_t base, size_t context) {
  return *(volatile u32 *)(base + VIRTUAL_KERNEL_BASE +
                           SIFIVE_INT_PRI_CLAIM_BASE + 0x1000 * context);
}

static void plic_complete_mm(size_t base, size_t context, u32 irq) {
  *(volatile u32 *)(base + VIRTUAL_KERNEL_BASE + SIFIVE_INT_PRI_CLAIM_BASE +
                    0x1000 * context) = irq;
}

void plic_turn_um(void) { plic.ops = &plic_um_ops; }

void plic_turn_mm(void) { plic.ops = &plic_mm_ops; }

void plic_init(size_t base, size_t size) { plic.ops->init(base, size); }

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