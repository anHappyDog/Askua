#include <dev/plic.h>
#include <dev/uart.h>
#include <mm/mm.h>
#include <printk.h>
#include <sbicall.h>
#include <smp.h>

static u8 _is_inited = 0;
u32 master_hartid = 0;

void _init(size_t hartid, void *dtbptr) {
  if (!_is_inited) {
    _is_inited = 1;
    master_hartid = SMP_GET_HARTID();
    uart_init();
    raw_heap_init();
    plic_init(SIFIVE_BASE_ADDR, SIFIVE_BASE_SIZE);
    mm_master(0x80000000, 0x80000000);

    printk("hartid = %d\n", hartid);
    printk("dtbptr = %016lx\n", dtbptr);
    while (1)
      ;
  } else {
    while (1)
      ;
  }
}