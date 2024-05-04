.section .boot._start, "ax", %progbits
.global _start
_start: 
    mv gp, a0
    li sp,0x80206000
    li t0, 0x1000
    mul t0, t0, gp
    sub sp, sp, t0
    j _preinit

.global _start_slave
_start_slave:
    mv gp, a0
    li sp,0x80206000
    li t0, 0x1000
    mul t0, t0, gp
    sub sp, sp, t0
    j _preinit_slave