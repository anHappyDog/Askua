#include <dev/plic.h>
#include <asm/csr.h>
#include <dev/uart.h>
#include <mm/mm.h>
#include <printk.h>
#include <sbicall.h>
#include <smp.h>
#include <trap.h>

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
    trap_init();
    sbi_set_timer(0x1000000 + read_time());
    printk("master hartid = %d\n", master_hartid);  
    while (1)
      ;
  } else {
    mm_slave();
    trap_init();
    printk("slave hartid = %d\n", hartid);
    sbi_set_timer(0x1000000 + read_time());
    while (1)
      ;
  }
}