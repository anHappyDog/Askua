#include <mm/mmu.h>
#include <types.h>

struct pre_heap {
  size_t start;
  size_t end;
  size_t current;
};

static __PREINIT__(.preheap.data) struct pre_heap heap = {
    .start = PREHEAP_BASE,
    .end = PREHEAP_BASE + PREHEAP_SZ,
    .current = PREHEAP_BASE,
};

size_t __PREINIT__(.preheap) pre_heap_alloc(size_t size, size_t align) {
  size_t ret = heap.current;
  if (ret + size > heap.end) {
    return 0;
  }
  ret = ROUNDUP(ret, align);
  heap.current = ret + size;
  return ret;
}
