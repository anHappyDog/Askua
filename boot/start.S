.section.text._boot, "ax", % progbits.global _start _start : mv gp, a0 la sp,
    __estack li t0, 0x10000 mul t0, t0, gp sub sp, sp, t0 j _init