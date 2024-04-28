#include <asm/csr.h>
#include <dev/uart.h>
#include <mm/mm.h>
#include <mm/pgtable.h>
#include <printk.h>
#include <virtio/virtio_blk.h>

extern size_t __stext, __etext, __sdata, __edata, __sbss, __ebss, __srodata,
    __erodata;
extern size_t __sraw, __eraw, __sstack, __estack;

pgd_t *kpgd;

static error_t kmapping_pte(pte_t *pte, size_t va, size_t pa, size_t size,
                            size_t perm) {
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

static error_t kmapping_pmd(pmd_t *pmdir, size_t va, size_t pa, size_t size,
                            size_t perm) {
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
    panic_on(kmapping_pte(pte, sva, spa, nm_sz, perm), "kmapping_pte failed");
    mapped_sz += nm_sz;
    sva += nm_sz;
    spa += nm_sz;
  }
  return 0;
}

static error_t kmapping_va2pa(pgd_t *pgdir, size_t va, size_t pa, size_t size,
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
    panic_on(kmapping_pmd(pmd, sva, spa, nm_sz, perm), "kmapping_pmd failed");
    mapped_sz += nm_sz;
    sva += nm_sz;
    spa += nm_sz;
  }
  return E_OK;
}

error_t kmapping(size_t mem_addr, size_t mem_size) {
  size_t ssbi = mem_addr, esbi = mem_addr + 0x200000;
  size_t stext = (size_t)&__stext, etext = (size_t)&__etext;
  size_t sdata = (size_t)&__sdata, edata = (size_t)&__edata;
  size_t sbss = (size_t)&__sbss, ebss = (size_t)&__ebss;
  size_t srodata = (size_t)&__srodata, erodata = (size_t)&__erodata;
  size_t sraw = (size_t)&__sraw, eraw = (size_t)&__eraw;
  size_t sstack = (size_t)&__sstack, estack = (size_t)&__estack;
  size_t lftmem = estack + PAGE_SIZE, lftmem_end = mem_addr + mem_size;
  kpgd = (pgd_t *)raw_heap_alloc(PAGE_SIZE, PAGE_SIZE);

  panic_on(kmapping_va2pa(kpgd, VIRTIO_BLK_ADDR, VIRTIO_BLK_ADDR, PAGE_SIZE,
                          PTE_R | PTE_W),
           "kmapping_va2pa failed");
  panic_on(kmapping_va2pa(kpgd, NS16550A_UART_BASE, NS16550A_UART_BASE,
                          PAGE_SIZE, PTE_R | PTE_W),
           "kmapping_va2pa failed");

  panic_on(kmapping_va2pa(kpgd, ssbi, ssbi, esbi - ssbi, PTE_R | PTE_W),
           "kmapping_va2pa failed");
  panic_on(kmapping_va2pa(kpgd, stext, stext, etext - stext, PTE_R | PTE_X),
           "kmapping_va2pa failed");
  panic_on(kmapping_va2pa(kpgd, sdata, sdata, edata - sdata, PTE_R | PTE_W),
           "kmapping_va2pa failed");
  panic_on(kmapping_va2pa(kpgd, sbss, sbss, ebss - sbss, PTE_R | PTE_W),
           "kmapping_va2pa failed");
  panic_on(kmapping_va2pa(kpgd, srodata, srodata, erodata - srodata, PTE_R),
           "kmapping_va2pa failed");
  panic_on(kmapping_va2pa(kpgd, sraw, sraw, eraw - sraw, PTE_R | PTE_W),
           "kmapping_va2pa failed");
  panic_on(kmapping_va2pa(kpgd, sstack, sstack, estack - sstack, PTE_R | PTE_W),
           "kmapping_va2pa failed");

  panic_on(kmapping_va2pa(kpgd, (size_t)kpgd, (size_t)kpgd, PAGE_SIZE,
                          PTE_R | PTE_W),
           "kmapping_va2pa failed");

  kpgd[VA_PGD_INDEX(VIRTUAL_KERNEL_BASE)] = kpgd[0] | PTE_V;
  kpgd[VA_PGD_INDEX(VIRTUAL_KERNEL_BASE) + 1] = kpgd[1] | PTE_V;
  kpgd[VA_PGD_INDEX(VIRTUAL_KERNEL_BASE) + 2] = kpgd[2] | PTE_V;
  write_satp(((size_t)kpgd >> PAGE_SHIFT) | SATP_SV39_MODE);
  uart_mm_mapped();
  printk("kmapping done\n");
  return E_OK;
}
