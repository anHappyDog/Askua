#ifndef __PAGE_H_
#define __PAGE_H_
#include <atomic.h>
#include <errors.h>
#include <list.h>
#include <lock/spinlock.h>

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
  struct list_head free_block_list;
  // used to link
  struct list_head page_list;
  atomic_t p_ref;
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

  spinlock_t lock;
};

typedef struct mpaging mpaging_t;

error_t mm_paging(size_t mem_base, size_t mem_size);

#endif // __PAGE_H_