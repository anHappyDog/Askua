#include <asm/csr.h>
#include <mm/mmu.h>
#include <mm/pgtable.h>
extern size_t __SECTION__(.text.rawheap)
    raw_heap_alloc(size_t size, size_t align);

extern size_t __text_start__[], __text_end__[];
extern size_t __rodata_start__[], __rodata_end__[];
extern size_t __data_start__[], __data_end__[];
extern size_t __bss_start__[], __bss_end__[];

pgd_t* kpgd = NULL;

static error_t __SECTION__(.text.kmmap)
    kmapping_pte(pte_t *pte, size_t va, size_t pa, size_t size, size_t perm) {
  size_t sva = ROUNDDOWN(va, PAGE_SIZE), spa = ROUNDDOWN(pa, PAGE_SIZE);
  size_t mapped_sz = 0, nm_sz = 0;
  size_t eva = ROUNDUP(va + size, PAGE_SIZE);
  size_t start = VA_PTE_INDEX(ROUNDDOWN(sva, PAGE_SIZE)),
         end = VA_PTE_INDEX(ROUNDUP(eva, PAGE_SIZE)), ind = 0;
  for (ind = start; ind < end; ++ind) {
    pte[ind] = PA_TO_PTE(spa + mapped_sz) | perm | PTE_V;
    mapped_sz += PAGE_SIZE;
  }
  return 0;
}

static error_t __SECTION__(.text.kmmap)
    kmapping_pmd(pmd_t *pmdir, size_t va, size_t pa, size_t size, size_t perm) {
  size_t pmd_sz = 1 << PMD_SHIFT;
  size_t sva = ROUNDDOWN(va, PAGE_SIZE), spa = ROUNDDOWN(pa, PAGE_SIZE);
  size_t mapped_sz = 0, nm_sz = 0;
  size_t eva = ROUNDUP(va + size, PAGE_SIZE);
  size_t start = VA_PMD_INDEX(ROUNDDOWN(sva, pmd_sz)),
         end = VA_PMD_INDEX(ROUNDUP(eva, pmd_sz)), ind = 0;
  for (ind = start; ind < end; ++ind) {
    if (!(pmdir[ind] & PTE_V)) {
      pmdir[ind] = PA_TO_PMD(raw_heap_alloc(PAGE_SIZE, PAGE_SIZE)) | PTE_V;
    }
    pte_t *pte = (pte_t *)PMD_TO_PA(pmdir[ind]);
    nm_sz = mapped_sz + pmd_sz < size ? pmd_sz : size - mapped_sz;
    kmapping_pte(pte, sva, spa, nm_sz, perm);
    mapped_sz += nm_sz;
    sva += nm_sz;
    spa += nm_sz;
  }
  return 0;
}

static error_t __SECTION__(.text.kmmap)
    kmapping_va2pa(pgd_t *pgdir, size_t va, size_t pa, size_t size,
                   size_t perm) {
  size_t pgd_sz = 1 << PGD_SHIFT;
  size_t sva = ROUNDDOWN(va, PAGE_SIZE), spa = ROUNDDOWN(pa, PAGE_SIZE);
  size_t mapped_sz = 0, nm_sz = 0;
  size_t eva = ROUNDUP(va + size, PAGE_SIZE),
         epa = ROUNDUP(pa + size, PAGE_SIZE);
  size_t start = VA_PGD_INDEX(ROUNDDOWN(sva, pgd_sz)),
         end = VA_PGD_INDEX(ROUNDUP(eva, pgd_sz)), ind = 0;
  for (ind = start; ind < end; ++ind) {
    if (!(pgdir[ind] & PTE_V)) {
      pgdir[ind] = PA_TO_PGD(raw_heap_alloc(PAGE_SIZE, PAGE_SIZE)) | PTE_V;
    }
    pmd_t *pmd = (pmd_t *)PGD_TO_PA(pgdir[ind]);
    nm_sz = mapped_sz + pgd_sz < size ? pgd_sz : size - mapped_sz;
    kmapping_pmd(pmd, sva, spa, nm_sz, perm);
    mapped_sz += nm_sz;
    sva += nm_sz;
    spa += nm_sz;
  }
  return E_OK;
}

error_t __SECTION__(.text.kmmap) kmapping(size_t mem_addr, size_t mem_size) {
  size_t t = 0;
  asm volatile("csrr %0, satp" : "=r"(t) : : "memory");
  pgd_t *pgd = (pgd_t *)(VIRTUAL_KERNEL_BASE |
                         ((t & ~SATP_SV39_MODE) << PAGE_SHIFT));
  kmapping_va2pa(pgd, (size_t)__text_start__,
                 (size_t)__text_start__ & ~VIRTUAL_KERNEL_BASE,
                 (size_t)__text_end__ - (size_t)__text_start__, PTE_R | PTE_X);
  kmapping_va2pa(pgd, (size_t)__rodata_start__,
                 (size_t)__rodata_start__ & ~VIRTUAL_KERNEL_BASE,
                 (size_t)__rodata_end__ - (size_t)__rodata_start__, PTE_R);
  kmapping_va2pa(pgd, (size_t)__data_start__,
                 (size_t)__data_start__ & ~VIRTUAL_KERNEL_BASE,
                 (size_t)__data_end__ - (size_t)__data_start__, PTE_R | PTE_W);
  kmapping_va2pa(pgd, (size_t)__bss_start__,
                 (size_t)__bss_start__ & ~VIRTUAL_KERNEL_BASE,
                 (size_t)__bss_end__ - (size_t)__bss_start__, PTE_R | PTE_W);
  kpgd = pgd;
  return E_OK;
}
