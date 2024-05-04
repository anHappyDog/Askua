#include <virtio/virtio_blk.h>
#include <asm/csr.h>
#include <dev/plic.h>
#include <dev/rtc.h>
#include <dev/uart.h>
#include <mm/mm.h>
#include <printk.h>
#include <sbicall.h>
#include <smp.h>
#include <trap.h>

extern void __PREINIT__(.pmm) kpre_mapping(void);
extern error_t __SECTION__(.text.kmmap)
    kmapping(size_t mem_addr, size_t mem_size);

void __PREINIT__(.preinit) __NORETURN__ _preinit(size_t hartid, void *dtbptr) {
  kpre_mapping();
  __TO_JUMPER__(hartid, dtbptr);
  __DEADLOOP__
}

void __PREINIT__(.preinit) __NORETURN__
    _preinit_slave(size_t hartid, void *dtbptr) {
  __TO_JUMPER__(hartid, dtbptr);
  __DEADLOOP__
}

static int inited = 0;

void __SECTION__(.text.init) __NORETURN__ _init(size_t hartid, void *dtbptr) {
  // Dead loop
  if (inited == 0) {
    inited = 1;
    enable_trap();
    plic_init(SIFIVE_BASE_ADDR, SIFIVE_BASE_SIZE);
    rtc_init(GOLDFISH_RTC_BASE, GOLDFISH_RTC_SIZE);
    virtio_blk_init(VIRTIO_BLK_ADDR | VIRTUAL_KERNEL_BASE);
    printk("hartid: %d\n", hartid);
  } else {
  }
  __DEADLOOP__
}

void __SECTION__(.text.jumper) __NORETURN__
    _jumper(size_t hartid, void *dtbptr) {
  // Dead loop
  __JUMPER_RESTORE_STACK(hartid);
  kmapping(0x80000000, 0x10000000);
  _init(hartid, dtbptr);
  __DEADLOOP__
}
