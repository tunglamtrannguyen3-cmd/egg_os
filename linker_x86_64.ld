/* linker_x86_64.ld */

ENTRY(_start)

SECTIONS
{
    /* Physical load base address at 1 MB */
    . = 0x100000;

    .text : ALIGN(4K)
    {
        /* MUST be first so QEMU/GRUB sees Multiboot2 header */
        KEEP(*(.multiboot2_header))
        *(.text .text.*)
    }

    .rodata : ALIGN(4K)
    {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4K)
    {
        *(.data .data.*)
    }

    .bss : ALIGN(4K)
    {
        sbss = .;   /* Export start of BSS symbol for boot.s */
        *(COMMON)
        *(.bss .bss.*)
        ebss = .;   /* Export end of BSS symbol for boot.s */
    }
}

