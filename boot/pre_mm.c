#include <asm/csr.h>
#include <mm/mm.h>
#include <mm/pgtable.h>

extern size_t __PREINIT__(.preheap) pre_heap_alloc(size_t size, size_t align);

static error_t __PREINIT__(.pmm)
    kpre_mm_pte(pte_t *pte, size_t va, size_t pa, size_t size, size_t perm) {
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

static error_t __PREINIT__(.pmm)
    kpre_mm_pmd(pmd_t *pmdir, size_t va, size_t pa, size_t size, size_t perm) {
  size_t pmd_sz = 1 << PMD_SHIFT;
  size_t sva = ROUNDDOWN(va, PAGE_SIZE), spa = ROUNDDOWN(pa, PAGE_SIZE);
  size_t mapped_sz = 0, nm_sz = 0;
  size_t eva = ROUNDUP(va + size, PAGE_SIZE);
  size_t start = VA_PMD_INDEX(ROUNDDOWN(sva, pmd_sz)),
         end = VA_PMD_INDEX(ROUNDUP(eva, pmd_sz)), ind = 0;
  for (ind = start; ind < end; ++ind) {
    if (!(pmdir[ind] & PTE_V)) {
      pmdir[ind] = PA_TO_PMD(pre_heap_alloc(PAGE_SIZE, PAGE_SIZE)) | PTE_V;
    }
    pte_t *pte = (pte_t *)PMD_TO_PA(pmdir[ind]);
    nm_sz = mapped_sz + pmd_sz < size ? pmd_sz : size - mapped_sz;
    kpre_mm_pte(pte, sva, spa, nm_sz, perm);
    mapped_sz += nm_sz;
    sva += nm_sz;
    spa += nm_sz;
  }
  return 0;
}

static error_t __PREINIT__(.pmm)
    kpre_mm_va2pa(pgd_t *pgdir, size_t va, size_t pa, size_t size,
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
      pgdir[ind] = PA_TO_PGD(pre_heap_alloc(PAGE_SIZE, PAGE_SIZE)) | PTE_V;
    }
    pmd_t *pmd = (pmd_t *)PGD_TO_PA(pgdir[ind]);
    nm_sz = mapped_sz + pgd_sz < size ? pgd_sz : size - mapped_sz;
    kpre_mm_pmd(pmd, sva, spa, nm_sz, perm);
    mapped_sz += nm_sz;
    sva += nm_sz;
    spa += nm_sz;
  }
  return E_OK;
}

void __PREINIT__(.pmm) kpre_mapping(void) {
  pgd_t *pre_pgd = (pgd_t *)pre_heap_alloc(PAGE_SIZE, PAGE_SIZE);
  kpre_mm_va2pa(pre_pgd, PREHEAP_BASE, PREHEAP_BASE, PREHEAP_SZ,
                 PTE_R | PTE_W | PTE_G);
  kpre_mm_va2pa(pre_pgd, PRESTACK_BASE, PRESTACK_BASE, PRESTACK_SZ,
                 PTE_R | PTE_W | PTE_G);
  kpre_mm_va2pa(pre_pgd, PREINIT_BASE, PREINIT_BASE, PAGE_SIZE,
                 PTE_R | PTE_X | PTE_G);

  kpre_mm_va2pa(pre_pgd, PHYSICAL_STACK_BASE | VIRTUAL_KERNEL_BASE,
                 PHYSICAL_STACK_BASE, STACK_SZ, PTE_R | PTE_W | PTE_G);
  kpre_mm_va2pa(pre_pgd, RAWHEAP_BASE | VIRTUAL_KERNEL_BASE, RAWHEAP_BASE,
                 RAWHEAP_SZ, PTE_R | PTE_W | PTE_G);

  kpre_mm_va2pa(pre_pgd, PHYSICAL_JUMPER_BASE |  VIRTUAL_KERNEL_BASE,
                 PHYSICAL_JUMPER_BASE, PAGE_SIZE, PTE_R | PTE_X | PTE_G);
  kpre_mm_va2pa(pre_pgd, PHYSICAL_JUMPER_DATA_BASE | VIRTUAL_KERNEL_BASE,
                 PHYSICAL_JUMPER_DATA_BASE, PAGE_SIZE, PTE_R | PTE_W | PTE_G);
  kpre_mm_va2pa(pre_pgd, (size_t)pre_pgd | VIRTUAL_KERNEL_BASE,
                 (size_t)pre_pgd, PAGE_SIZE, PTE_R | PTE_W | PTE_G);
  write_pre_satp((((size_t)pre_pgd) >> PAGE_SHIFT) | SATP_SV39_MODE);
}