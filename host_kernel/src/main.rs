#![no_std]
#![no_main]
#![allow(dead_code)]

mod drivers;
mod hypercall;
mod memory;
mod vmm;

use common::ModulesRequest;
use core::panic::PanicInfo;

#[used]
#[unsafe(link_section = ".requests")]
static MODULES_REQUEST: ModulesRequest = ModulesRequest::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    memory::ept::init_host_memory();

    let mut guest_entry_addr = 0u64;
    let mut app_ramdisk_addr = 0u64;
    let mut app_ramdisk_size = 0u64;

    if let Some(resp) = MODULES_REQUEST.response() {
        let modules = unsafe {
            core::slice::from_raw_parts(resp.modules, resp.module_count as usize)
        };

        // Module 0: Security Kernel, Module 1: Offline App Ramdisk
        if let Some(kernel_mod) = modules.get(0) {
            guest_entry_addr = kernel_mod.base_address;
        }
        if let Some(app_mod) = modules.get(1) {
            app_ramdisk_addr = app_mod.base_address;
            app_ramdisk_size = app_mod.size;
        }
    }

    if guest_entry_addr == 0 {
        loop {
            x86_64::instructions::hlt();
        }
    }

    if vmm::vmx::enable_vmx().is_err() {
        loop {
            x86_64::instructions::hlt();
        }
    }

    let mut vcpu = vmm::VCpu::new(0);
    vcpu.setup_vmcs(guest_entry_addr);

    let _ = (app_ramdisk_addr, app_ramdisk_size);

    loop {
        let exit_reason = vcpu.run();
        vmm::vmexit::handle_vmexit(&mut vcpu, exit_reason);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}