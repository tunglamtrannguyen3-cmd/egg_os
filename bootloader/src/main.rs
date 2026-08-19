#![no_std]
#![no_main]

mod arch;
mod elf;
mod fs;
mod memory;

use arch::x86_64::{disable_interrupts, jump_to_kernel};
use uefi::prelude::*;
use uefi::table::boot::MemoryType;
use uefi::Status;

/// Magic identifier passed to host_kernel in register r8 / 3rd argument
const BOOT_MAGIC: u64 = 0x4547_475F_4F53_0000; // "EGG_OS"

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // 1. Initialize UEFI helper services
    if uefi::helpers::init(&mut system_table).is_err() {
        return Status::ABORTED;
    }

    // 2. Load host_kernel executable from boot media
    let kernel_bytes = match fs::load_kernel_file(image_handle, &mut system_table) {
        Ok(bytes) => bytes,
        Err(_) => return Status::NOT_FOUND,
    };

    // 3. Parse ELF header and map kernel pages
    let host_elf = match elf::parse_and_map(&kernel_bytes) {
        Ok(elf) => elf,
        Err(_) => return Status::LOAD_ERROR,
    };

    // 4. Set up kernel stack and BootInfo structure
    let stack_top = match memory::allocate_kernel_stack() {
        Ok(top) => top,
        Err(_) => return Status::OUT_OF_RESOURCES,
    };

    let boot_info_ptr = match memory::build_boot_info(&mut system_table) {
        Ok(ptr) => ptr,
        Err(_) => return Status::OUT_OF_RESOURCES,
    };

    // 5. Exit UEFI Boot Services (uefi 0.28 requires MemoryType allocation tracking)
    let (_runtime_table, _mmap) = unsafe { system_table.exit_boot_services(MemoryType::LOADER_DATA) };

    // 6. Silence CPU interrupts and transfer control to host_kernel
    unsafe {
        disable_interrupts();
        jump_to_kernel(host_elf.entry_point, stack_top, BOOT_MAGIC, boot_info_ptr);
    }
}
