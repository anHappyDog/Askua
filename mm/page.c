#include <mm/mmu.h>
#include <mm/page.h>
#include <mm/pgtable.h>
#include <printk.h>
mpaging_t mpging = {
    .pm_base = 0, .pm_size = 0, .pages = NULL, .npages = 0, .pb_ops = NULL};

extern struct pb_operations_struct buddy_pb_ops;
extern size_t raw_heap_alloc(size_t size, size_t align);
extern size_t __skernel, __ekernel;
error_t mm_paging(size_t mem_base, size_t mem_size) {
  size_t skernel = (size_t)&__skernel;
  size_t ekernel = (size_t)&__ekernel;
  size_t free_npage = (mem_size - ekernel) >> PAGE_SHIFT;
  mpging.pm_base = mem_base;
  mpging.pm_size = mem_size;
  mpging.npages = mem_size / PAGE_SIZE;
  mpging.pages =
      (page_t *)(VIRTUAL_KERNEL_BASE |
                 raw_heap_alloc(sizeof(page_t) * mpging.npages, PAGE_SIZE));
  panic_on(!mpging.pages, "failed to allocate pages for paging");
  for (size_t i = 0; i < mpging.npages; ++i) {
    if (i * PAGE_SIZE < ekernel) {
      mpging.pages[i].p_virtaddr =
          mpging.pm_base + VIRTUAL_KERNEL_BASE + i * PAGE_SIZE;

      mpging.pages[i].p_flags = PAGE_RESERVED;
    } else {
      mpging.pages[i].p_virtaddr =
          mpging.pm_base + VIRTUAL_PAGING_BASE + i * PAGE_SIZE;
      mpging.pages[i].p_flags = PAGE_FREE;
    }
    mpging.pages[i].p_physaddr = mpging.pm_base + i * PAGE_SIZE;
    mpging.pages[i].p_nr = i;
    INIT_LIST_HEAD(&mpging.pages[i].pb_list);
    INIT_LIST_HEAD(&mpging.pages[i].p_list);
  }
  mpging.pb_ops = &buddy_pb_ops;

  mpging.pb_ops->alloc_init(mpging.pages + (ekernel >> PAGE_SHIFT), free_npage);
  return E_OK;
}

size_t alloc_pages(size_t order) {
  if (mpging.pb_ops) {
    return mpging.pb_ops->alloc_pb(order);
  }
  panic("no page allocator");
}

error_t free_pages(size_t addr, size_t order) {
  if (mpging.pb_ops) {
    return mpging.pb_ops->free_pb(addr, order);
  }
  panic("no page allocator");
}

error_t alloc_pages_zeroed(size_t order) {
  if (mpging.pb_ops) {
    return mpging.pb_ops->alloc_pb_zeroed(order);
  }
  panic("no page allocator");
}
