/* ============================================================================
   switch.S - UEFI → Kernel Transition (x86_64)
   ============================================================================
   Corrected version
   ============================================================================ */

.section .text
.global jump_to_kernel
.global disable_interrupts

jump_to_kernel:
    /* 1. Disable interrupts */
    cli

    /* 2. Preserve kernel_entry (rdi) before it gets clobbered */
    mov     %rdi, %rax

    /* 3. Set up new kernel stack (16-byte aligned) */
    mov     %rsi, %rsp
    and     $-16, %rsp

    /* 4. Clear direction flag */
    cld

    /* 5. Zero out unused registers */
    xor     %rbx, %rbx
    xor     %rbp, %rbp
    xor     %r8,  %r8
    xor     %r9,  %r9
    xor     %r10, %r10
    xor     %r11, %r11
    xor     %r12, %r12
    xor     %r13, %r13
    xor     %r14, %r14
    xor     %r15, %r15

    /* 6. Prepare System V ABI arguments for the kernel
          rdi = magic
          rsi = boot_info_ptr
    */
    mov     %rdx, %rdi
    mov     %rcx, %rsi

    /* 7. Jump to the real kernel entry point */
    jmp     *%rax

disable_interrupts:
    cli
    ret

.hang:
    cli
    hlt
    jmp     .hang

