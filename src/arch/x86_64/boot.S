/* ==========================================
   x86_64/boot.s - Multiboot2 + Stronger Early Boot
   ========================================== */

.section .multiboot2_header, "a"
.align 8
header_start:
    .long 0xe85250d6
    .long 0
    .long header_end - header_start
    .long -(0xe85250d6 + 0 + (header_end - header_start))

    /* End tag */
    .word 0
    .word 0
    .long 8
header_end:

.section .text
.global _start
.extern kernel_main
.extern sbss
.extern ebss

_start:
    cli
    cld

    /* Lưu Multiboot */
    movq %rax, %r8                  /* magic */
    movq %rbx, %r9                  /* info */

    /* Setup stack */
    leaq stack_top(%rip), %rsp
    andq $-16, %rsp                 /* Align stack 16 bytes */

    /* Clear BSS */
    leaq sbss(%rip), %rdi
    leaq ebss(%rip), %rcx
    subq %rdi, %rcx
    xorq %rax, %rax
    shrq $3, %rcx                   /* Clear 8 bytes at a time */
    rep stosq

    /* Gọi kernel_main(magic, info) */
    movq %r8, %rdi
    movq %r9, %rsi
    call kernel_main

.hang:
    cli
    hlt
    jmp .hang

.section .bss
.align 16
stack_bottom:
    .skip 65536                     /* 64KB stack */
stack_top:
