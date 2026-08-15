#[inline(always)]
pub unsafe fn vmxon(vmxon_phys_addr: u64) -> Result<(), ()> {
    let err: u8;
    core::arch::asm!(
        "vmxon [{0}]",
        "setc {1}",
        in(reg) &vmxon_phys_addr,
        out(reg_byte) err,
    );
    if err != 0 { Err(()) } else { Ok(()) }
}

#[inline(always)]
pub unsafe fn vmptrld(vmcs_phys_addr: u64) -> Result<(), ()> {
    let err: u8;
    core::arch::asm!(
        "vmptrld [{0}]",
        "setc {1}",
        in(reg) &vmcs_phys_addr,
        out(reg_byte) err,
    );
    if err != 0 { Err(()) } else { Ok(()) }
}

#[inline(always)]
pub unsafe fn vmwrite(field: u64, val: u64) {
    core::arch::asm!(
        "vmwrite {0}, {1}",
        in(reg) field,
        in(reg) val,
    );
}

pub fn enable_vmx() -> Result<(), ()> {
    // 1. Enable CR4.VMXE bit
    unsafe {
        let mut cr4: u64;
        core::arch::asm!("mov {0}, cr4", out(reg) cr4);
        cr4 |= 1 << 13; // Set VMXE bit
        core::arch::asm!("mov cr4, {0}", in(reg) cr4);
    }
    Ok(())
}

pub fn init_vmcs_region() {
    // Allocate 4KB physical page for VMCS and load pointer via vmptrld
}

