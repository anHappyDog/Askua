#include <mm/mm.h>
#include <printk.h>

extern pgd_t *kpgd;

void mm_master(size_t mem_base, size_t mem_size) {
  panic_on(kmapping(mem_base, mem_size), "kmapping failed!");
  panic_on(mm_paging(mem_base, mem_size), "mm paging failed!");
}

void mm_slave(void) {}