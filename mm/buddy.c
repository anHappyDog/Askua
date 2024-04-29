#include <list.h>
#include <mm/mmu.h>
#include <mm/page.h>

#define BUDDY_MAX_ORDER 6

static error_t buddy_alloc_init(page_t *free_pages, size_t npages);
static size_t buddy_alloc_pages(size_t order);
static error_t buddy_free_pages(size_t addr, size_t order);
static size_t buddy_alloc_pages_zeroed(size_t order);

struct pb_operations_struct buddy_pb_ops = {
    .alloc_init = buddy_alloc_init,
    .alloc_pb = buddy_alloc_pages,
    .free_pb = buddy_free_pages,
    .alloc_pb_zeroed = buddy_alloc_pages_zeroed,
    .free_pages = NULL,
    .free_base = 0,
};

struct pb_desc {
  size_t order;
  size_t allocated_count;
  size_t total_count;
  size_t petion;
  size_t nr_pages; // means one block need how many pages.
  struct list_head pb_list;
};

static struct pb_desc pb_descs[BUDDY_MAX_ORDER] = {
    {0, 0, 0, 60, 1, LIST_HEAD_INIT(pb_descs[0].pb_list)},
    {1, 0, 0, 20, 2, LIST_HEAD_INIT(pb_descs[1].pb_list)},
    {2, 0, 0, 10, 4, LIST_HEAD_INIT(pb_descs[2].pb_list)},
    {3, 0, 0, 5, 8, LIST_HEAD_INIT(pb_descs[3].pb_list)},
    {4, 0, 0, 3, 16, LIST_HEAD_INIT(pb_descs[4].pb_list)},
    {5, 0, 0, 2, 32, LIST_HEAD_INIT(pb_descs[5].pb_list)},
};

static size_t buddy_alloc_pages(size_t order) {
  if (order >= BUDDY_MAX_ORDER) {
    return 0;
  }
  struct pb_desc *pb_desc = &pb_descs[order];
  if (pb_desc->allocated_count >= pb_desc->total_count) {
    return 0;
  }
  struct page *page = list_entry(&pb_desc->pb_list, struct page, pb_list);
  pb_desc->allocated_count++;
  return page->p_virtaddr;
}

static error_t buddy_free_pages(size_t addr, size_t order) {
  if (order >= BUDDY_MAX_ORDER) {
    return E_INVAL;
  }
  struct pb_desc *pb_desc = &pb_descs[order];
  if (pb_desc->allocated_count == 0) {
    return E_INVAL;
  }
  struct page *page =
      buddy_pb_ops.free_pages + ((addr - buddy_pb_ops.free_base) >> PAGE_SHIFT);
  page->p_flags |= PAGE_FREE;
  pb_desc->allocated_count--;
  list_add_tail(&page->pb_list, &pb_desc->pb_list);
  return E_OK;
}

static size_t buddy_alloc_pages_zeroed(size_t order) {
  size_t addr = buddy_alloc_pages(order);
  if (addr) {
    memset((void *)addr, 0, 1 << (order + PAGE_SHIFT));
  }
  return addr;
}

static error_t buddy_alloc_init(page_t *free_pages, size_t npages) {
  i16 i = 0;
  size_t j = 0;
  buddy_pb_ops.free_pages = free_pages;
  buddy_pb_ops.free_base = free_pages->p_virtaddr;
  for (i = BUDDY_MAX_ORDER - 1; i >= 0; --i) {
    pb_descs[i].total_count = npages * pb_descs[i].petion / 100;
    for (j = 0; j < pb_descs[i].total_count; ++j) {
        free_pages->p_flags = PAGE_FREE;
        list_add_tail(&free_pages->pb_list, &pb_descs[i].pb_list);
        free_pages+= pb_descs[i].nr_pages;
    }
  }
  return E_OK;
}
