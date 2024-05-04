OUTPUT_ARCH(riscv)
ENTRY(_start)


HIDDEN(PREINIT_BASE = 0x80200000);
HIDDEN(KERNEL_BASE = 0xFFFFFFF000000000);
HIDDEN(PRESTACK_SZ = 0x4000);
HIDDEN(PREHEAP_SZ = 0x10000);
HIDDEN(STACK_SZ = 0x40000);
HIDDEN(RAWHEAP_SZ = 0x800000);
SECTIONS {
    . = PREINIT_BASE;

    .boot : {
        *(.boot._start)
        *(.boot .boot.*)
    }
    
    . = ALIGN(4K);
    . += 4K;
    ASSERT(SIZEOF(.boot) <= 4K, "boot section too large")
    .prestack (NOLOAD) : {
        . += PRESTACK_SZ;
    }

    .preheap (NOLOAD) : {
        . += PREHEAP_SZ;
    }

    .stack ((KERNEL_BASE | LOADADDR(.preheap)) + PREHEAP_SZ) (NOLOAD) : AT(LOADADDR(.preheap) + PREHEAP_SZ) {
        . += STACK_SZ;
    }

    .rawheap (NOLOAD) : {
        . += RAWHEAP_SZ;
    }

    .jumper : {
        *(.text.jumper)
        *(.text.rawheap)
        *(.jumper.kmmap)
        
    }
    . = ALIGN(4K);
    .jumper.data : {
        *(.jumper.data .jumper.data.*)
        *(.got .got.*)
    }
    . = ALIGN(4K);
    ASSERT(SIZEOF(.jumper) <= 4K, "jumper section too large")
    ASSERT(SIZEOF(.jumper.data) <= 4K, "jumper data section too large")
    __text_start__ = .;
    .text  :  {
        *(.text.init)
        *(.text .text.*)
    }
    . = ALIGN(4K);
    __text_end__ = .;
    . += 4K;
    __rodata_start__ = .;
    .rodata : {
        *(.rodata .rodata.*)
    }
    . = ALIGN(4K);
    __rodata_end__ = .;
    . += 4K;
    __data_start__ = .;
    .data : {
        *(.data .data.*)
    }
    . = ALIGN(4K);
    __data_end__ = .;
    . += 4K;
    __bss_start__ = .;
    .bss : {
        *(.bss .bss.*)
    }
    . = ALIGN(4K);
    __bss_end__ = .;
    __kernel_end__ = .;
}

