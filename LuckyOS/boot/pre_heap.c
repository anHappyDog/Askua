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
  ret = ROUNDUP(ret, align);
  if (ret + size > heap.end) {
    return 0;
  }
  for (size_t i = 0; i < size; ++i) {
    *(volatile u8 *)ret = 0;
    ret++;
  }
  heap.current = ret;
  return ret - size;
}
