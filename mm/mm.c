#include <asm/csr.h>
#include <mm/mm.h>
#include <printk.h>
#include <sbicall.h>
#include <smp.h>

extern pgd_t *kpgd;
extern u32 master_hartid;
void (*master_core_jumper)(u32) = NULL;
void (*slave_core_jumper)(void) = NULL;

static void mv_mc_to_hs(u32 master_hartid) {
  for (int i = 0; i < 4; ++i) {
    if (i == master_hartid) {
      continue;
    }
    // sbi_hart_start(i, 0x80200000, 0);
  }
  RESTORE_MC_MC_TO_HS_STACK
}

static void mv_sc_to_hs(void) {}

void mm_master(size_t mem_base, size_t mem_size) {
  panic_on(kmapping(mem_base, mem_size), "kmapping failed!");
  master_core_jumper =
      (void (*)(u32))((size_t)mv_mc_to_hs | VIRTUAL_KERNEL_BASE);
  slave_core_jumper =
      (void (*)(void))((size_t)mv_sc_to_hs | VIRTUAL_KERNEL_BASE);
  master_core_jumper(master_hartid);
  panic_on(mm_paging(mem_base, mem_size), "mm paging failed!");
  RESTORE_MM_MASTER_STACK
}

void mm_slave(void) {
  write_satp(((size_t)kpgd >> PAGE_SHIFT) | SATP_SV39_MODE);
  if (!slave_core_jumper) {
    panic("slave core jumper is not set!");
  }
  slave_core_jumper();
  printk("slave core %d mmu enabled!\n", SMP_GET_HARTID());
}
