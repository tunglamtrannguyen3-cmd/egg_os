#![no_std]
#![no_main]

mod hypercall;
mod memory;
mod vmm;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. Initialize Host Physical Memory & EPT Page Tables
    memory::ept::init_host_memory();

    // 2. Initialize Hardware Virtualization (Intel VT-x / VMX)
    if let Err(_err) = vmm::vmx::enable_vmx() {
        // Fallback or log error via serial if VMX fails
        loop { x86_64::instructions::hlt(); }
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

