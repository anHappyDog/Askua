#include <mm/pgtable.h>
#include <asm/csr.h>
#include <dev/uart.h>
#include <virtio/virtio_blk.h>
#include <mm/mm.h>
#include <printk.h>

extern size_t __stext, __etext, __sdata, __edata, __sbss, __ebss, __srodata, __erodata;
extern size_t __sraw, __eraw, __sstack, __estack;


pgd_t* kpgd;

static error_t kmapping_pgd(size_t va, size_t pa, size_t size, size_t perm)
{
    return 0;
}

static error_t kmapping_pmd(size_t va, size_t pa, size_t size, size_t perm)
{
    return 0;
}

static error_t kmapping_pte(size_t va, size_t pa, size_t size, size_t perm)
{
    return 0;
}

static error_t kmapping_va2pa(pgd_t *pgdir, size_t va, size_t pa, size_t size, size_t perm)
{
    return 0;
}






error_t kmapping(size_t mem_addr, size_t mem_size)
{
    size_t ssbi = mem_addr,esbi = mem_addr + 0x200000;
    size_t stext = (size_t)&__stext, etext = (size_t)&__etext;
    size_t sdata = (size_t)&__sdata, edata = (size_t)&__edata;
    size_t sbss = (size_t)&__sbss, ebss = (size_t)&__ebss;
    size_t srodata = (size_t)&__srodata, erodata = (size_t)&__erodata;
    size_t sraw = (size_t)&__sraw, eraw = (size_t)&__eraw;
    size_t sstack = (size_t)&__sstack, estack = (size_t)&__estack;
    size_t lftmem = estack + PAGE_SIZE,lftmem_end = mem_addr + mem_size;
    kpgd = (pgd_t *)raw_heap_alloc(PAGE_SIZE, PAGE_SIZE);


    panic_on(kmapping_va2pa(kpgd, VIRTIO_BLK_ADDR, VIRTIO_BLK_ADDR, PAGE_SIZE, PTE_R | PTE_W), "kmapping_va2pa failed");
    panic_on(kmapping_va2pa(kpgd, NS16550A_UART_BASE, NS16550A_UART_BASE, PAGE_SIZE, PTE_R | PTE_W), "kmapping_va2pa failed");

    panic_on(kmapping_va2pa(kpgd, ssbi, ssbi, esbi - ssbi, PTE_R | PTE_W), "kmapping_va2pa failed");
    panic_on(kmapping_va2pa(kpgd, stext, stext, etext - stext, PTE_R | PTE_X), "kmapping_va2pa failed");
    panic_on(kmapping_va2pa(kpgd, sdata, sdata, edata - sdata, PTE_R | PTE_W), "kmapping_va2pa failed");
    panic_on(kmapping_va2pa(kpgd, sbss, sbss, ebss - sbss, PTE_R | PTE_W), "kmapping_va2pa failed");
    panic_on(kmapping_va2pa(kpgd, srodata, srodata, erodata - srodata, PTE_R), "kmapping_va2pa failed");
    panic_on(kmapping_va2pa(kpgd, sraw, sraw, eraw - sraw, PTE_R | PTE_W), "kmapping_va2pa failed");
    panic_on(kmapping_va2pa(kpgd, sstack, sstack, estack - sstack, PTE_R | PTE_W), "kmapping_va2pa failed");
    
    kpgd[VA_PGD_INDEX(VIRTUAL_KERNEL_BASE)] = kpgd[0] | PTE_V;
    kpgd[VA_PGD_INDEX(VIRTUAL_KERNEL_BASE) + 1] = kpgd[1] | PTE_V;
    kpgd[VA_PGD_INDEX(VIRTUAL_KERNEL_BASE) + 2] = kpgd[2] | PTE_V;

    write_satp((size_t)kpgd | SATP_SV39_MODE);
    return E_OK;
}

