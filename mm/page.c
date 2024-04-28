#include <mm/page.h>
#include <mm/pgtable.h>
mpaging_t mpging = {
    .pm_base = 0,
    .pm_size = 0,
    .pages = NULL,
    .npages = 0,
    .lock = 0,
};


error_t mm_paging(size_t mem_base, size_t mem_size) {
    mpging.pm_base = mem_base;
    mpging.pm_size = mem_size;
    mpging.npages = mem_size / PAGE_SIZE;
    mpging.pages = (page_t *) (VIRTUAL_KERNEL_BASE | raw_heap_alloc(sizeof(page_t) * mpging.npages, PAGE_SIZE));
    

    return E_OK;
}


