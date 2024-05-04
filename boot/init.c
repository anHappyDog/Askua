#include <asm/csr.h>
#include <dev/plic.h>
#include <dev/rtc.h>
#include <dev/uart.h>
#include <mm/mm.h>
#include <printk.h>
#include <sbicall.h>
#include <smp.h>
#include <trap.h>
#include <virtio/virtio_blk.h>

static pgd_t *__PREINIT_DATA__ pre_pgd = NULL;
extern void __PREINIT_START__ _start_slave(void);
extern pgd_t *__PREINIT__() kpre_mapping(void);
extern error_t __JUMPER_KMMAP__ kmapping(size_t mem_addr, size_t mem_size);

void __PREINIT__() __NORETURN__ _preinit(size_t hartid, void *dtbptr) {
  pre_pgd = kpre_mapping();
  __TO_JUMPER__(hartid, dtbptr);
  __DEADLOOP__
}

void __PREINIT__() __NORETURN__ _preinit_slave(size_t hartid, void *dtbptr) {
  __SLAVE_ENABLE_MMU__(pre_pgd);
  __TO_JUMPER__(hartid, dtbptr);
  __DEADLOOP__
}

void __TEXT_INIT__ __NORETURN__ _init(size_t hartid, void *dtbptr) {
  enable_trap();
  plic_init(SIFIVE_BASE_ADDR, SIFIVE_BASE_SIZE);
  rtc_init(GOLDFISH_RTC_BASE, GOLDFISH_RTC_SIZE);
  mm_paging(MEM_BASE, MEM_SIZE);
  virtio_blk_init(VIRTIO_BLK_ADDR | VIRTUAL_KERNEL_BASE);
  printk("hartid: %d\n", hartid);
  for (int i = 0; i < CORE; i++) {
    if (i != hartid) {
      sbi_hart_start(i, (uintptr_t)_start_slave, (uintptr_t)dtbptr);
    }
  }
  sbi_set_timer(read_time() + 1000000);
  __DEADLOOP__
}

void __TEXT_INIT__ __NORETURN__ _init_slave(size_t hartid) {
  enable_trap();
  sbi_set_timer(read_time() + 1000000);
  __DEADLOOP__
}

void __JUMPER__ __NORETURN__ _jumper(size_t hartid, void *dtbptr) {
  __JUMPER_RESTORE_STACK(hartid);
  __JUMP_TO_INIT__(
      {
        kmapping(MEM_BASE, MEM_SIZE);
        _init(hartid, dtbptr);
      },
      { _init_slave(hartid); });
}
