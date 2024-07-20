.section .text._start,"ax",%progbits
.globl _start
_start:
	la sp, boot_stack_top
	li t0, 131072
	mul t0, t0, a0
	sub sp, sp, t0
	j _init

.section .bss.stack
.globl boot_stack
boot_stack:
    .space 131072 * 4
.global boot_stack_top
boot_stack_top:
