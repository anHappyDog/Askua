/*
 * Used to alloc 1 page per alloc in master core before mmu starts. 
 * the allocated page will never be freed,because it is mainly used for kernel's data structure.
 * When mmu starts,the left pages will be managed by mmu,and this function will not be used. 
*/
#include <mm/mm.h>
extern size_t __sraw;
extern size_t __eraw;

struct RawHeapAllocator {
    size_t start;
    size_t end;
    size_t current;
};

static struct RawHeapAllocator raw_heap_allocator = {
    .start = 0,
    .end = 0,
    .current = 0,
};

void raw_heap_init(void) {
    raw_heap_allocator.start = (size_t)&__sraw;
    raw_heap_allocator.end = (size_t)&__eraw;
    raw_heap_allocator.current = raw_heap_allocator.start;
}

size_t raw_heap_alloc(size_t size, size_t align) {
    size_t ret = 0;
    size_t current = raw_heap_allocator.current;
    size_t aligned = ROUNDUP(current, align);
    size_t new_current = aligned + size;
    if (new_current <= raw_heap_allocator.end) {
        raw_heap_allocator.current = new_current;
        ret = aligned;
    memset((void *)ret, 0, size);
    }
    return ret;
}





