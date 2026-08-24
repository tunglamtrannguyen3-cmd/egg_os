#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

mod arch;
mod elf;
mod fs;
mod memory;

use uefi::boot::{self, MemoryType};
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    // 1. Initialize UEFI runtime helpers and heap allocator
    uefi::helpers::init().unwrap();

    #[cfg(test)]
    test_main();

    // Acquire current image handle
    let image_handle = boot::image_handle();

    // 1. Read host_kernel using fs module
    let kernel_bytes = match fs::load_kernel_file(image_handle) {
        Ok(bytes) => bytes,
        Err(_) => {
            log::error!("Failed to load kernel file!");
            loop { core::hint::spin_loop(); }
        }
    };

    // 2. Parse ELF headers
    let loaded_elf = match elf::parse_and_map(&kernel_bytes) {
        Ok(elf) => elf,
        Err(_) => {
            log::error!("Failed to parse/map ELF binary!");
            loop { core::hint::spin_loop(); }
        }
    };

    drop(kernel_bytes);

    // 3. Allocate stack & boot info using memory module
    let stack_top = match memory::allocate_kernel_stack() {
        Ok(top) => top,
        Err(_) => {
            log::error!("Failed to allocate kernel stack!");
            loop { core::hint::spin_loop(); }
        }
    };

    let boot_info_ptr = match memory::build_boot_info() {
        Ok(ptr) => ptr,
        Err(_) => {
            log::error!("Failed to build boot info!");
            loop { core::hint::spin_loop(); }
        }
    };

    // 4. Exit boot services passing Some(MemoryType::LOADER_DATA)
    let _memory_map = unsafe { boot::exit_boot_services(Some(MemoryType::LOADER_DATA)) };

    // 5. Jump to host kernel using assembly routine
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

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}