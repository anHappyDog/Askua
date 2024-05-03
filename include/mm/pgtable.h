#ifndef __PGTABLE_H_
#define __PGTABLE_H_
#include <errors.h>
#include <lib/string.h>
#include <mm/mmu.h>
#include <mm/page.h>
#include <types.h>

typedef uint64_t pgd_t;
typedef uint64_t pmd_t;
typedef uint64_t pte_t;

#define PGD_SIZE PAGE_SIZE
#define PMD_SIZE PAGE_SIZE
#define PTE_SIZE PAGE_SIZE

#define PGD_BITS 9
#define PMD_BITS 9
#define PTE_BITS 9
#define PERM_BTIS 10

#define PTE_SHIFT 12
#define PMD_SHIFT (PTE_SHIFT + PTE_BITS)
#define PGD_SHIFT (PMD_SHIFT + PMD_BITS)

#define PGD_ORDER 0
#define PMD_ORDER 0
#define PTE_ORDER 0

#define PTE_V (1 << 0)
#define PTE_R (1 << 1)
#define PTE_W (1 << 2)
#define PTE_X (1 << 3)
#define PTE_U (1 << 4)
#define PTE_G (1 << 5)
#define PTE_A (1 << 6)
#define PTE_D (1 << 7)

#define SATP_BASE_MODE 0
#define SATP_SV39_MODE (8UL << 60)
#define SATP_SV48_MODE (9UL << 60)
#define SATP_SV57_MODE (10UL << 60)
#define SATP_SV64_MODE (11UL << 60)

#define VA_PGD_INDEX(va) (((va) >> PGD_SHIFT) & ((1 << PGD_BITS) - 1))
#define VA_PMD_INDEX(va) (((va) >> PMD_SHIFT) & ((1 << PMD_BITS) - 1))
#define VA_PTE_INDEX(va) (((va) >> PTE_SHIFT) & ((1 << PTE_BITS) - 1))

#define PA_TO_PGD(va) (((va) >> PAGE_SHIFT) << PERM_BTIS)
#define PA_TO_PMD(va) (((va) >> PAGE_SHIFT) << PERM_BTIS)
#define PA_TO_PTE(va) (((va) >> PAGE_SHIFT) << PERM_BTIS)

#define PGD_TO_PA(va) (((va) >> PERM_BTIS) << PAGE_SHIFT)
#define PMD_TO_PA(va) (((va) >> PERM_BTIS) << PAGE_SHIFT)
#define PTE_TO_PA(va) (((va) >> PERM_BTIS) << PAGE_SHIFT)


#endif // __PGTABLE_H_