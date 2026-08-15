/* ==========================================
   switch.S - VMX Host ↔ Guest Context Switch
   ========================================== */

.section .text
.global vmx_switch_to_guest
.global vmx_vmexit_handler

/* -----------------------------------------------------
   vmx_switch_to_guest
   Input (from Rust):
     rdi = pointer to Guest register state buffer
   ----------------------------------------------------- */
vmx_switch_to_guest:
    /* 1. Save Host general-purpose registers */
    push %rax
    push %rbx
    push %rcx
    push %rdx
    push %rsi
    push %rdi
    push %rbp
    push %r8
    push %r9
    push %r10
    push %r11
    push %r12
    push %r13
    push %r14
    push %r15

    /* 2. Save Host RSP into VMCS (HOST_RSP field = 0x6C14) */
    mov %rsp, %rax
    vmwrite %rax, $0x6C14          /* VMCS_HOST_RSP */

    /* 3. Load Guest registers from buffer (rdi) */
    mov 0x00(%rdi), %rax
    mov 0x08(%rdi), %rbx
    mov 0x10(%rdi), %rcx
    mov 0x18(%rdi), %rdx
    mov 0x20(%rdi), %rsi
    /* rdi sẽ load sau */
    mov 0x30(%rdi), %rbp
    mov 0x38(%rdi), %r8
    mov 0x40(%rdi), %r9
    mov 0x48(%rdi), %r10
    mov 0x50(%rdi), %r11
    mov 0x58(%rdi), %r12
    mov 0x60(%rdi), %r13
    mov 0x68(%rdi), %r14
    mov 0x70(%rdi), %r15
    mov 0x28(%rdi), %rdi           /* load rdi cuối */

    /* 4. Enter Guest */
    vmresume
    jbe vmx_launch_failed          /* CF=1 hoặc ZF=1 → lỗi */

    /* Nếu vmresume fail lần đầu thì thử vmlaunch */
    vmlaunch
    jbe vmx_launch_failed

    /* Không bao giờ tới đây nếu thành công */
    jmp vmx_launch_failed

/* -----------------------------------------------------
   vmx_vmexit_handler
   Được set làm Host RIP trong VMCS
   ----------------------------------------------------- */
vmx_vmexit_handler:
    /* 1. Save Guest registers vào buffer
          (Giả sử buffer pointer đang được lưu ở một nơi cố định
           hoặc dùng GS/FS base. Ở đây dùng cách đơn giản: push tạm) */

    /* Tạm thời push Guest regs lên stack để không mất */
    push %rax
    push %rbx
 dual %rcx
    push %rdx
    push %rsi
    push %rdi
    push %rbp
    push %r8
    push %r9
    push %r10
    push %r11
    push %r12
    push %r13
    push %r14
    push %r15

    /* 2. Restore Host registers (đã push trước khi vào Guest) */
    /* Lúc này stack đang có:
       [Guest regs vừa push]
       [Host regs đã push từ trước]
    */

    /* Bỏ Guest regs ra (hoặc copy vào buffer nếu cần) */
    add $15*8, %rsp                /* bỏ 15 thanh ghi Guest */

    /* 3. Restore Host regs */
    pop %r15
    pop %r14
    pop %r13
    pop %r12
    pop %r11
    pop %r10
    pop %r9
    pop %r8
    pop %rbp
    pop %rdi
    pop %rsi
    pop %rdx
    pop %rcx
    pop %rbx
    pop %rax

    /* 4. Quay về Rust */
    ret

vmx_launch_failed:
    /* Xử lý lỗi đơn giản: treo hoặc trả lỗi */
    hlt
    jmp vmx_launch_failed
