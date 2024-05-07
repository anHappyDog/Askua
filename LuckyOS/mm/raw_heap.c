#include <mm/mmu.h>
#include <types.h>

struct raw_heap {
  size_t start;
  size_t end;
  size_t current;
};

static struct raw_heap __SECTION__(.jumper.data) raw_heap = {
    .start = RAWHEAP_BASE,
    .end = RAWHEAP_BASE + RAWHEAP_SZ,
    .current = RAWHEAP_BASE,
};

size_t __SECTION__(.text.rawheap) raw_heap_alloc(size_t size, size_t align) {
  size_t ret = raw_heap.current;
  ret = ROUNDUP(ret, align);

  if (ret + size > raw_heap.end) {
    return 0;
  }

  raw_heap.current = ret + size;
  return ret;
}
