#include <asm/csr.h>
#include <dev/plic.h>
#include <dev/rtc.h>
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
    rtc_init(GOLDFISH_RTC_BASE, GOLDFISH_RTC_SIZE);
    plic_init(SIFIVE_BASE_ADDR, SIFIVE_BASE_SIZE);
    raw_heap_init();
    mm_master(0x80000000, 0x10000000);
    enable_trap();
    sbi_set_timer(0x1000000 + read_time());
    printk("alarm is %016lx,alarm status is %x\n", rtc_read_alarm(),
           rtc_alarm_status());
    printk("master hartid = %d,time is %016lx\n", master_hartid,
           rtc_read_time());
    while (1)
      ;
  } else {

    mm_slave();
    enable_trap();
    printk("slave hartid = %d\n", hartid);
    sbi_set_timer(0x1000000 + read_time());
    while (1)
      ;
  }
}