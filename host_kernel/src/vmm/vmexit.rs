use crate::println;
use crate::vmm::VCpu;
use crate::vmm::vmx::{vmread, vmwrite};

// --- Basic Exit Reasons (Intel SDM Vol. 3C, Appendix C) ---
const EXIT_REASON_EXCEPTION_NMI: u64 = 0;
const EXIT_REASON_CPUID: u64 = 10;
const EXIT_REASON_VMCALL: u64 = 18;
const EXIT_REASON_CR_ACCESS: u64 = 28;
const EXIT_REASON_EPT_VIOLATION: u64 = 48;

// --- VMCS Read Field Encodings ---
const GUEST_RIP: u32 = 0x0000681E;
const GUEST_RSP: u32 = 0x0000681C;
const GUEST_CR2: u32 = 0x00006822;
const VM_EXIT_INSTRUCTION_LEN: u32 = 0x0000440C;
const EXIT_QUALIFICATION: u32 = 0x00006400;

/// Main dispatcher for VM-Exits from `virtual_kernel`.
pub fn handle_vmexit(vcpu: &mut VCpu, exit_reason: u64) {
    // Mask basic exit reason (bits 0-15) to strip VM-Entry failure flags
    let basic_reason = exit_reason & 0xFFFF;

    match basic_reason {
        EXIT_REASON_EXCEPTION_NMI => handle_exception(vcpu),
        EXIT_REASON_EPT_VIOLATION => handle_ept_violation(vcpu),
        EXIT_REASON_VMCALL => handle_vmcall(vcpu),
        EXIT_REASON_CPUID => handle_cpuid(vcpu),
        EXIT_REASON_CR_ACCESS => handle_cr_access(vcpu),
        _ => {
            let rip = vmread(GUEST_RIP).unwrap_or(0);
            let rsp = vmread(GUEST_RSP).unwrap_or(0);
            panic!(
                "Unhandled VM-Exit Reason: {:#x} at Guest RIP: {:#x}, RSP: {:#x}",
                basic_reason, rip, rsp
            );
        }
    }
}

/// Handles trapped guest exceptions (such as #PF or #DF before they trigger a triple fault).
fn handle_exception(_vcpu: &mut VCpu) {
    let rip = vmread(GUEST_RIP).unwrap_or(0);
    let rsp = vmread(GUEST_RSP).unwrap_or(0);
    let qual = vmread(EXIT_QUALIFICATION).unwrap_or(0);
    let cr2 = vmread(GUEST_CR2).unwrap_or(0);

    // If EXCEPTION_BITMAP captured a Page Fault (#PF)
    println!("--- GUEST EXCEPTION TRAPPED ---");
    println!("Guest RIP: {:#018x}", rip);
    println!("Guest RSP: {:#018x}", rsp);
    println!("Faulting Address (CR2): {:#018x}", cr2);
    println!("Exit Qualification:    {:#018x}", qual);

    panic!("Guest executed an unhandled exception in virtual_kernel!");
}

/// Handles EPT Violations (Guest Physical Address page faults).
fn handle_ept_violation(_vcpu: &mut VCpu) {
    let rip = vmread(GUEST_RIP).unwrap_or(0);
    let guest_phys_addr = vmread(EXIT_QUALIFICATION).unwrap_or(0);

    println!("--- EPT VIOLATION (Missing Physical Frame) ---");
    println!("Guest RIP:               {:#018x}", rip);
    println!("Faulting Guest PhysAddr: {:#018x}", guest_phys_addr);

    panic!("Virtual kernel accessed unmapped host memory!");
}

/// Handles hypercalls executed via `vmcall` instruction inside `virtual_kernel`.
fn handle_vmcall(_vcpu: &mut VCpu) {
    // Process hypercall arguments stored in guest registers here...
    
    // Advance Guest RIP past the 3-byte `vmcall` instruction to prevent infinite exit loop
    advance_guest_rip();
}

/// Emulates CPUID instruction execution.
fn handle_cpuid(_vcpu: &mut VCpu) {
    // Populate guest RAX, RBX, RCX, RDX register values for requested feature flags...

    advance_guest_rip();
}

/// Handles Control Register (CR0/CR3/CR4) reads or writes.
fn handle_cr_access(_vcpu: &mut VCpu) {
    // Process CR register access qualification...

    advance_guest_rip();
}

/// Moves Guest RIP forward by the length of the instruction that caused the exit.
fn advance_guest_rip() {
    let current_rip = vmread(GUEST_RIP).expect("Failed to read GUEST_RIP");
    let instruction_len = vmread(VM_EXIT_INSTRUCTION_LEN).expect("Failed to read instruction length");
    vmwrite(GUEST_RIP, current_rip + instruction_len).expect("Failed to write GUEST_RIP");
}