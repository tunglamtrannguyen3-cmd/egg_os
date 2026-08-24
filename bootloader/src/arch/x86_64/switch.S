/* ============================================================================
   switch.S - UEFI → Kernel Transition (x86_64)
   Hardened version
   ============================================================================ */

.section .text
.global jump_to_kernel

/*
    System V AMD64 Calling Convention:

    On entry to this function:
        rdi = kernel_entry      (u64)
        rsi = new_stack         (u64)
        rdx = magic             (u64)
        rcx = boot_info_ptr     (u64)

    On entry to the kernel:
        rdi = magic
        rsi = boot_info_ptr
*/
jump_to_kernel:
    /* === 1. Disable interrupts immediately === */
    cli

    /* === 2. Save kernel entry point (rdi will be overwritten) === */
    mov     rax, rdi

    /* === 3. Switch to the new kernel stack and force 16-byte alignment === */
    mov     rsp, rsi
    and     rsp, -16                 /* RSP must be 0 mod 16 */

    /* === 4. ABI requirements === */
    cld                              /* Direction flag must be clear */

    /* === 5. Zero caller-saved / unused registers for cleanliness === */
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

    /* === 6. Prepare arguments for the kernel === */
    mov     rdi, rdx                 /* rdi = magic */
    mov     rsi, rcx                 /* rsi = boot_info_ptr */

    /* === 7. Jump to kernel (never returns) === */
    jmp     rax

/* Safety hang if anything goes wrong */
.hang:
    cli
    hlt
    jmp     .hang