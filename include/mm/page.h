#ifndef __PAGE_H_
#define __PAGE_H_
#include <atomic.h>
#include <errors.h>
#include <lib/string.h>
#include <list.h>
#include <lock/spinlock.h>
#include <mm/mmu.h>

#define PAGE_SHIFT 12
#define PAGE_SIZE (1 << PAGE_SHIFT)

#define PAGE_FREE 0
#define PAGE_RESERVED 1
#define PAGE_USED 2
#define PAGE_CONTIGOUS                                                         \
  4 // may be useful for the multi physical page allocating.

struct page {
  // used for muti-contiguous-page allocating.
  // see the first page in the 'block' as a node.
  // only the node page are in it.
  struct list_head pb_list;
  // used to link
  struct list_head p_list;
  atomic_t p_ref;
  size_t p_virtaddr;
  size_t p_physaddr;
  uint32_t p_flags;
  size_t p_nr; // number
};

typedef struct page page_t;

struct mpaging {
  size_t pm_base;
  size_t pm_size;
  page_t *pages;
  size_t npages;
  struct pb_operations_struct *pb_ops;
};

struct pb_operations_struct {
  error_t (*alloc_init)(page_t *free_pages, size_t npages);
  size_t (*alloc_pb)(size_t order);
  error_t (*free_pb)(size_t addr, size_t order);
  size_t (*alloc_pb_zeroed)(size_t order);
  page_t *free_pages;
  size_t free_base; // virtual address
};

typedef struct mpaging mpaging_t;


#define PAGE_VA_TO_PA(va) ((va) & ~VIRTUAL_KERNEL_BASE)


error_t mm_paging(size_t mem_base, size_t mem_size);

size_t alloc_pages(size_t order);
error_t free_pages(size_t addr, size_t order);
#endif // __PAGE_H_