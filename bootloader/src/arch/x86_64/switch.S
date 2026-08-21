/* ============================================================================
   switch.S - UEFI → Kernel Transition (x86_64)
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
    mov     %rdi, %rax

    /* Set new stack (16-byte aligned) */
    mov     %rsi, %rsp
    and     $-16, %rsp

    /* Clear direction flag */
    cld

    /* Zero unused registers */
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

    /* System V ABI arguments for kernel:
       rdi = magic
       rsi = boot_info_ptr
    */
    mov     %rdx, %rdi
    mov     %rcx, %rsi

    /* Jump to kernel (never returns) */
    jmp     *%rax

.hang:
    cli
    hlt
    jmp     .hang