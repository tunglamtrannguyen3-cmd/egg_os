/* security_kernel/linker_x86_64.ld */
ENTRY(_start)

SECTIONS
{
    . = 0x100000;

    .text : {
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
    }
}

