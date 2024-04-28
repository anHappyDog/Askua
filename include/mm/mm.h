#ifndef __MM_H__
#define __MM_H__
#include <mm/page.h>
#include <mm/pgtable.h>
#include <types.h>

// raw.c
size_t raw_heap_alloc(size_t size, size_t align);
void raw_heap_init(void);

void mm_master(size_t mem_base, size_t mem_size);
void mm_slave(void);
#endif // __MM_H__