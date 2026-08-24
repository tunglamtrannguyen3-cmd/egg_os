#![no_std]
#![no_main]
#![allow(dead_code)]

mod drivers;
mod hypercall;
mod memory;
mod vmm;

use common::ModulesRequest;
use core::panic::PanicInfo;

/// 4KB-aligned physical page frame required for VMXON
#[repr(C, align(4096))]
struct AlignedPage([u8; 4096]);

static mut VMXON_REGION: AlignedPage = AlignedPage([0; 4096]);

#[used]
#[unsafe(link_section = ".requests")]
static MODULES_REQUEST: ModulesRequest = ModulesRequest::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize serial output for host_kernel println debugging
    drivers::serial::SerialPort::init();
    
    memory::ept::init_host_memory();

    let mut guest_entry_addr = 0u64;
    let mut app_ramdisk_addr = 0u64;
    let mut app_ramdisk_size = 0u64;

    if let Some(resp) = MODULES_REQUEST.response() {
        let modules = unsafe {
            core::slice::from_raw_parts(resp.modules, resp.module_count as usize)
        };

        // Module 0: Virtual Kernel, Module 1: Offline App Ramdisk
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

    // Obtain the 4KB-aligned physical address of the VMXON region
    let vmxon_paddr = core::ptr::addr_of_mut!(VMXON_REGION) as u64;

    if vmm::vmx::enable_vmx(vmxon_paddr).is_err() {
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