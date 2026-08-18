#![no_std]
#![no_main]
#![allow(dead_code)]

mod drivers;
mod hypercall;
mod memory;
mod vmm;

use core::panic::PanicInfo;
use limine::request::ModulesRequest;

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
        for module in resp.modules().iter() {
            let cmd = module.cmdline();

            // THE KERNEL DEV HACK:
            // Since the crate made the fields private, we cast the struct
            // to a raw 64-bit integer pointer and read the memory directly.
            // Layout: [0] = revision, [1] = address, [2] = size
            let file_ptr = *module as *const _ as *const u64;
            let raw_addr = unsafe { *file_ptr.add(1) };
            let raw_size = unsafe { *file_ptr.add(2) };

            if cmd.contains("security_kernel") {
                guest_entry_addr = raw_addr;
            } else if cmd.contains("offline_app") {
                app_ramdisk_addr = raw_addr;
                app_ramdisk_size = raw_size;
            }
        }
    }

    if vmm::vmx::enable_vmx().is_err() {
        loop {
            x86_64::instructions::hlt();
        }
    }

    let mut vcpu = vmm::VCpu::new(0);
    vcpu.setup_vmcs(guest_entry_addr);

    // Pass ramdisk location via registers or shared state later
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
