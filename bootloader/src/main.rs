#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

/// Entry point called by the BIOS/UEFI bootloader stage
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialization code goes here

    #[cfg(test)]
    test_main();

    loop {
        // Halts CPU execution safely without relying on unsafe blocks
        core::hint::spin_loop();
    }
}

/// Panic handler required for no_std binaries
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Custom test runner to prevent dependencies on std::test
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}