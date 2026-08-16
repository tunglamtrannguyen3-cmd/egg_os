// src/main.rs

#![no_std]
#![no_main]

pub mod arch;
pub mod capability;
pub mod drivers;
pub mod ipc;
pub mod memory;
pub mod sched;
pub mod syscall;
pub mod ui;

use capability::{allocate_with_capability, CapRights};
use ipc::IpcChannel;
use ui::DisplayMode;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel_main();

    loop {
        #[cfg(target_arch = "x86_64")]
        core::arch::x86_64::_mm_pause();
    }
}

pub fn kernel_main() {
    crate::arch::log("=====================================================\n");
    crate::arch::log("               EggOS Microkernel v1.0                \n");
    crate::arch::log("   Adaptive Memory • Capability Gates • Dynamic UI   \n");
    crate::arch::log("=====================================================\n\n");

    memory::init();
    capability::init();
    sched::init();
    ipc::init();
    syscall::init();
    ui::init();

    crate::arch::log("\n-----------------------------------------------------\n");
    crate::arch::log("[EggOS Kernel]: Boot Sequence Completed Successfully.\n");
    crate::arch::log("-----------------------------------------------------\n\n");

    crate::arch::log("[Self-Test 1]: Rendering Dynamic Resolution Status Bar...\n");
    ui::render_desktop(DisplayMode::DEFAULT_1080P, 100, false);
    crate::arch::log(" -> Framebuffer render successful.\n\n");

    crate::arch::log("[Self-Test 2]: Testing Adaptive Capped Memory Stream...\n");
    let test_bytes = 150 * 1024;
    match allocate_with_capability(2, test_bytes, CapRights::READ_WRITE) {
        Ok((ptr, token)) => {
            crate::arch::log(" -> Allocation & Capability Token Minting: SUCCESS\n");

            match capability::GLOBAL_CAP_GATE.lock().verify_access(
                token.id,
                ptr,
                test_bytes,
                CapRights::READ_WRITE,
            ) {
                Ok(_) => crate::arch::log(" -> Capability Gate Verification: GRANTED\n"),
                Err(err) => crate::arch::log(err),
            }

            let channel = IpcChannel::new(1, token.id);
            if let Err(err) = channel.stream_payload(test_bytes) {
                crate::arch::log(err);
            }
        }
        Err(err) => crate::arch::log(err),
    }

    crate::arch::log("\n=====================================================\n");
    crate::arch::log(" [EggOS Kernel]: All systems online. Ready for tasks. \n");
    crate::arch::log("=====================================================\n");
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        #[cfg(target_arch = "x86_64")]
        core::arch::x86_64::_mm_pause();
    }
}
