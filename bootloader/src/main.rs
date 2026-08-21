#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod arch;
mod elf;
mod fs;
mod memory;

use uefi::prelude::*;
use uefi::table::boot::MemoryType;

#[entry]
fn efi_main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    #[cfg(test)]
    test_main();

    // 1. Read host_kernel using fs module
    let kernel_bytes = match fs::load_kernel_file(image_handle, &mut system_table) {
        Ok(bytes) => bytes,
        Err(_) => return Status::ABORTED,
    };

    // 2. Parse ELF headers
    let loaded_elf = match elf::parse_and_map(&kernel_bytes) {
        Ok(elf) => elf,
        Err(_) => return Status::ABORTED,
    };

    drop(kernel_bytes);

    // 3. Allocate stack & boot info using memory module
    let stack_top = match memory::allocate_kernel_stack() {
        Ok(top) => top,
        Err(_) => return Status::ABORTED,
    };

    let boot_info_ptr = match memory::build_boot_info(&mut system_table) {
        Ok(ptr) => ptr,
        Err(_) => return Status::ABORTED,
    };

    // 4. Exit boot services
    let (_runtime_services, _memory_map) = system_table.exit_boot_services(MemoryType::LOADER_DATA);

    // 5. Jump to host kernel using the assembly routine in arch::x86_64
    const BOOT_MAGIC: u64 = 0x2026_0000;

    unsafe {
        arch::x86_64::jump_to_kernel(
            loaded_elf.entry_point,
            stack_top,
            BOOT_MAGIC,
            boot_info_ptr as *const common::BootInfo,
        );
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}