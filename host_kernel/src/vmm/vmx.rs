use core::arch::asm;

// --- Low-Level Assembly Helpers ---

#[inline]
pub unsafe fn rdmsr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    asm!(
        "rdmsr",
        in("ecx") msr,
        out("eax") low,
        out("edx") high,
        options(nomem, nostack, preserves_flags)
    );
    ((high as u64) << 32) | (low as u64)
}

#[inline]
pub unsafe fn wrmsr(msr: u32, val: u64) {
    let low = val as u32;
    let high = (val >> 32) as u32;
    asm!(
        "wrmsr",
        in("ecx") msr,
        in("eax") low,
        in("edx") high,
        options(nomem, nostack, preserves_flags)
    );
}

#[inline]
pub unsafe fn cr0_read() -> u64 {
    let val: u64;
    asm!(
        "mov {}, cr0",
        out(reg) val,
        options(nomem, nostack, preserves_flags)
    );
    val
}

#[inline]
pub unsafe fn cr4_read() -> u64 {
    let val: u64;
    asm!(
        "mov {}, cr4",
        out(reg) val,
        options(nomem, nostack, preserves_flags)
    );
    val
}

#[inline]
pub unsafe fn cr4_write(val: u64) {
    asm!(
        "mov cr4, {}",
        in(reg) val,
        options(nomem, nostack, preserves_flags)
    );
}

// --- MSR Addresses ---
const IA32_FEATURE_CONTROL: u32 = 0x3A;
const IA32_VMX_BASIC: u32 = 0x480;

// --- Control Flags ---
const CR4_VMXE: u64 = 1 << 13;

// --- VMCS Encoding Constants ---
// 16-Bit Guest State Fields
const GUEST_ES_SELECTOR: u32 = 0x00000800;
const GUEST_CS_SELECTOR: u32 = 0x00000802;
const GUEST_SS_SELECTOR: u32 = 0x00000804;
const GUEST_DS_SELECTOR: u32 = 0x00000806;
const GUEST_FS_SELECTOR: u32 = 0x00000808;
const GUEST_GS_SELECTOR: u32 = 0x0000080A;
const GUEST_LDTR_SELECTOR: u32 = 0x0000080C;
const GUEST_TR_SELECTOR: u32 = 0x0000080E;

// 64-Bit Control Fields
const EPT_POINTER: u32 = 0x0000201A;

// 32-Bit Control Fields
const PIN_BASED_VM_EXEC_CONTROL: u32 = 0x00004000;
const CPU_BASED_VM_EXEC_CONTROL: u32 = 0x00004002;
const EXCEPTION_BITMAP: u32 = 0x00004004;
const VM_EXIT_CONTROLS: u32 = 0x0000400C;
const VM_ENTRY_CONTROLS: u32 = 0x00004012;

// Natural-Width Guest State Fields
const GUEST_CR0: u32 = 0x00006800;
const GUEST_CR3: u32 = 0x00006802;
const GUEST_CR4: u32 = 0x00006804;
const GUEST_RSP: u32 = 0x0000681C;
const GUEST_RIP: u32 = 0x0000681E;
const GUEST_RFLAGS: u32 = 0x00006820;

// Natural-Width Host State Fields
const HOST_CR0: u32 = 0x00006C00;
const HOST_CR3: u32 = 0x00006C02;
const HOST_CR4: u32 = 0x00006C04;
const HOST_RSP: u32 = 0x00006C14;
const HOST_RIP: u32 = 0x00006C16;

/// Reads VMX revision ID from IA32_VMX_BASIC MSR.
fn get_vmx_revision_id() -> u32 {
    let vmx_basic = unsafe { rdmsr(IA32_VMX_BASIC) };
    (vmx_basic & 0xFFFF_FFFF) as u32
}

/// Executes a raw `vmwrite` instruction.
pub fn vmwrite(field: u32, value: u64) -> Result<(), ()> {
    let status: u8;
    unsafe {
        asm!(
            "vmwrite {0}, {1}",
            "setc {2}",
            in(reg) field as u64,
            in(reg) value,
            out(reg_byte) status,
        );
    }
    if status == 0 { Ok(()) } else { Err(()) }
}

/// Executes a raw `vmread` instruction.
pub fn vmread(field: u32) -> Result<u64, ()> {
    let value: u64;
    let status: u8;
    unsafe {
        asm!(
            "vmread {0}, {1}",
            "setc {2}",
            out(reg) value,
            in(reg) field as u64,
            out(reg_byte) status,
        );
    }
    if status == 0 { Ok(value) } else { Err(()) }
}

/// Enables VMX execution mode on the CPU and enters root mode.
pub fn enable_vmx(vmxon_frame_paddr: u64) -> Result<(), ()> {
    // 1. Set CR4.VMXE bit
    unsafe {
        let cr4 = cr4_read();
        cr4_write(cr4 | CR4_VMXE);
    }

    // 2. Configure IA32_FEATURE_CONTROL MSR
    unsafe {
        let feature_control = rdmsr(IA32_FEATURE_CONTROL);
        if (feature_control & 1) == 0 {
            // Set Lock bit (bit 0) and Enable VMX outside SMX (bit 2)
            wrmsr(IA32_FEATURE_CONTROL, feature_control | 1 | (1 << 2));
        }
    }

    // 3. Populate VMXON region header with VMX Revision ID
    let revision_id = get_vmx_revision_id();
    unsafe {
        *(vmxon_frame_paddr as *mut u32) = revision_id;
    }

    // 4. Issue VMXON instruction
    let status: u8;
    unsafe {
        asm!(
            "vmxon [{0}]",
            "setc {1}",
            in(reg) &vmxon_frame_paddr,
            out(reg_byte) status,
        );
    }

    if status != 0 {
        return Err(());
    }

    Ok(())
}

/// Configures and activates the VMCS for `virtual_kernel`.
pub fn init_vmcs_region(
    vmcs_frame_paddr: u64,
    guest_entry_rip: u64,
    guest_stack_rsp: u64,
    guest_page_table_cr3: u64,
    ept_pointer_phys: u64,
    host_vmexit_rip: u64,
    host_stack_rsp: u64,
    host_cr3: u64,
) -> Result<(), ()> {
    // 1. Initialize VMCS revision header
    let revision_id = get_vmx_revision_id();
    unsafe {
        *(vmcs_frame_paddr as *mut u32) = revision_id;
    }

    // 2. Load active VMCS pointer
    let status: u8;
    unsafe {
        asm!(
            "vmptrld [{0}]",
            "setc {1}",
            in(reg) &vmcs_frame_paddr,
            out(reg_byte) status,
        );
    }
    if status != 0 {
        return Err(());
    }

    // --- 3. Guest State Setup ---
    // Minimal segment selectors for flat 64-bit kernel context
    vmwrite(GUEST_CS_SELECTOR, 0x08)?;
    vmwrite(GUEST_DS_SELECTOR, 0x10)?;
    vmwrite(GUEST_ES_SELECTOR, 0x10)?;
    vmwrite(GUEST_SS_SELECTOR, 0x10)?;
    vmwrite(GUEST_FS_SELECTOR, 0x00)?;
    vmwrite(GUEST_GS_SELECTOR, 0x00)?;
    vmwrite(GUEST_LDTR_SELECTOR, 0x00)?;
    vmwrite(GUEST_TR_SELECTOR, 0x18)?;

    // Execution Context
    vmwrite(GUEST_RIP, guest_entry_rip)?;
    vmwrite(GUEST_RSP, guest_stack_rsp)?;
    vmwrite(GUEST_RFLAGS, 0x02)?; // Bit 1 is always mandatory 1 in RFLAGS
    vmwrite(GUEST_CR0, 0x8001003B)?; // PE, MP, ET, NE, WP, PG enabled
    vmwrite(GUEST_CR3, guest_page_table_cr3)?;
    vmwrite(GUEST_CR4, 0x20)?; // PAE enabled (Required for 64-bit long mode)

    // --- 4. Host State Setup ---
    vmwrite(HOST_RIP, host_vmexit_rip)?;
    vmwrite(HOST_RSP, host_stack_rsp)?;
    vmwrite(HOST_CR0, unsafe { cr0_read() })?;
    vmwrite(HOST_CR3, host_cr3)?;
    vmwrite(HOST_CR4, unsafe { cr4_read() })?;

    // --- 5. Controls Setup ---
    vmwrite(PIN_BASED_VM_EXEC_CONTROL, 0)?;
    vmwrite(CPU_BASED_VM_EXEC_CONTROL, 0x80000000 | (1 << 31))?; // Activate secondary controls & EPT
    vmwrite(EPT_POINTER, ept_pointer_phys)?;
    
    // Intercept Double Faults (#DF, bit 8) and Page Faults (#PF, bit 14) to catch guest faults before triple fault reset
    vmwrite(EXCEPTION_BITMAP, (1 << 8) | (1 << 14))?;

    // Entry / Exit Controls for 64-bit Guest
    vmwrite(VM_EXIT_CONTROLS, (1 << 9) | (1 << 15))?; // 64-bit host, ack interrupt
    vmwrite(VM_ENTRY_CONTROLS, 1 << 9)?;             // IA-32e (64-bit) guest entry

    Ok(())
}

/// Launches the configured guest execution.
pub fn launch_guest() -> Result<(), ()> {
    let status: u8;
    unsafe {
        asm!(
            "vmlaunch",
            "setc {0}",
            out(reg_byte) status,
        );
    }
    // If VMLAUNCH succeeds, execution enters the guest and does not return here.
    // Return error if VMLAUNCH failed invalid state checks.
    if status != 0 { Err(()) } else { Ok(()) }
}