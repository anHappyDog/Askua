OUTPUT_ARCH(riscv)
ENTRY(_start)

HIDDEN(KERNEL_ENTRY_ADDR = 0x80200000);

SECTIONS {
    . = KERNEL_ENTRY_ADDR;

    PROVIDE(_stext = .);
    .text : {
        *(.text._start)
        *(.text .text.*)
		. = ALIGN(4K);
        PROVIDE(_etext = .);
        *(._trampoline)
        . = ALIGN(4K);
        PROVIDE(_etrampoline = .);
    }

    PROVIDE(_srodata = .);
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
		. = ALIGN(4K);
    	PROVIDE(_erodata = .);
    }

    PROVIDE(_sdata = .);
    .data : {
        *(.data .data.*)
		*(.sdata .sdata.*)
		/* . = ALIGN(4K); NO NEED TO ALIGN */
    	PROVIDE(_edata = .);
    }

	.stack (NOLOAD) : {
        *(.bss.stack)
    }
    .heap (NOLOAD) : {
        *(.bss.heap)
    }
    PROVIDE(_sbss = .);
    .bss : {
        *(.bss .bss.*)
		*(.sbss .sbss.*)
    	PROVIDE(_ebss = .);
    }
    PROVIDE(_ekernel = .);
}
