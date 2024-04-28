OUTPUT_ARCH(riscv)
ENTRY(_start)

HIDDEN(KERNEL_ENTRY_ADDR = 0x80200000);
HIDDEN(STACK_SIZE = 0x40000);
HIDDEN(RAW_SIZE = 0x20000);
SECTIONS {
    . = KERNEL_ENTRY_ADDR;
    __skernel = .;
    __stext = .;
    .text : {
        *(.text._boot)
        __sexcept = .;
        *(.text.exc_gen_entry);
        __eexcept = .;
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

    .raw (NOLOAD) : {
    __sraw = .;
    . += RAW_SIZE;
    __eraw = .;
    }
    
    .stack (NOLOAD) : {
        __sstack = .;
        . += STACK_SIZE;
        __estack = .;
    }
    __ekernel = .;
}
