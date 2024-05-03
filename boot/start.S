.section .boot._start, "ax", %progbits
.global _start
_start: 
    mv gp, a0
    li sp,0x80204000
    li t0, 0x1000
    mul t0, t0, gp
    sub sp, sp, t0
    j _preinit