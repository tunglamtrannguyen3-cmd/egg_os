#![no_std]
#![no_main]

mod hypercall;
mod memory;
mod vmm;

use crate::memory::ept::EptPageTable;
use crate::vmm::vmx::{vmptrld, vmwrite, vmxon};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. Initialize Host Physical Memory & EPT Page Tables
    memory::ept::init_host_memory();

    // 2. Initialize Hardware Virtualization (Intel VT-x / VMX)
    if let Err(_err) = vmm::vmx::enable_vmx() {
        // Fallback or log error via serial if VMX fails
        loop {
            x86_64::instructions::hlt();
        }
    }

    // 3. Create Virtual Machine Control Structure (VMCS) for Guest
    let mut vcpu = vmm::VCpu::new(0);
    vcpu.setup_vmcs();

    // 4. Hand off CPU control to the Guest Unikernel
    vcpu.run();

    // Reached if guest terminates
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init_hypervisor(vmxon_phys_addr: u64, vmcs_phys_addr: u64) {
    let ept = EptPageTable::new();
    // Test chunk loader logic (prepares guest memory / GUI framebuffer staging)
    let dummy_buf = [0u8; crate::memory::ept::PAGE_SIZE];
    let _ = crate::memory::ept::process_data_chunks(&dummy_buf, |_chunk, _offset| Ok(()));

    unsafe {
        let _ = vmxon(vmxon_phys_addr);
        let _ = vmptrld(vmcs_phys_addr);

        // Pass the raw pointer of EPT table to VMCS control field
        vmwrite(0x00004000, &ept as *const _ as u64);
    }
}
