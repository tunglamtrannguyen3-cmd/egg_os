/* ============================================================================
   switch.S - UEFI → Kernel Transition (x86_64)
   Intel Syntax version
   ============================================================================ */

.section .text
.global jump_to_kernel

/*
    jump_to_kernel(
        rdi = kernel_entry,
        rsi = new_stack,
        rdx = magic,
        rcx = boot_info_ptr
    )
*/
jump_to_kernel:
    /* Disable interrupts */
    cli

    /* Preserve kernel_entry */
    mov     rax, rdi

    /* Set new stack (16-byte aligned) */
    mov     rsp, rsi
    and     rsp, -16

    /* Clear direction flag */
    cld

    /* Zero unused registers */
    xor     rbx, rbx
    xor     rbp, rbp
    xor     r8,  r8
    xor     r9,  r9
    xor     r10, r10
    xor     r11, r11
    xor     r12, r12
    xor     r13, r13
    xor     r14, r14
    xor     r15, r15

    /* System V ABI arguments for kernel:
       rdi = magic
       rsi = boot_info_ptr
    */
    mov     rdi, rdx
    mov     rsi, rcx

    /* Jump to kernel (never returns) */
    jmp     rax

.hang:
    cli
    hlt
    jmp     .hang