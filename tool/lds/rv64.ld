OUTPUT_ARCH(riscv)
ENTRY(_start)

HIDDEN(KERNEL_ENTRY_ADDR = 0x80200000);
HIDDEN(STACK_SIZE = 0x40000);
SECTIONS {
    . = KERNEL_ENTRY_ADDR;
    __skernel = .;
    __stext = .;
    .text : {
        *(.text._boot)
        *(.text .text.*)
    }
    . = ALIGN(4K);
    __etext = .;
    . += 4K;
    __srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
    }
    . = ALIGN(4K);
    __erodata = .;
    . += 4K;
    __sdata = .;
    .data : {
        *(.data .data.*)
    }
    . = ALIGN(4K);
    __edata = .;
    __sbss = .;
    .bss : {
        *(.bss .bss.*)
    }
    . = ALIGN(4K);
    __ebss = .;
    . += 4K;

    .stack (NOLOAD) : {
        __sstack = .;
        . += STACK_SIZE;
        __estack = .;
    }
    __ekernel = .;
}
