// security_kernel/src/main.rs

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

use crate::ui::framebuffer::{DisplayMode, Framebuffer};
use capability::{allocate_with_capability, CapRights};
use ipc::IpcChannel;

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
    crate::arch::log("             EggOS Dual-Kernel Engine v1.0           \n");
    crate::arch::log("  Host VMX Hypervisor • Security Domain • GUI Engine \n");
    crate::arch::log("=====================================================\n\n");

    memory::init();
    capability::init();
    sched::init();
    ipc::init();
    syscall::init();

    // Initialize GUI Framebuffer via Host-mapped MMIO
    let mut fb = Framebuffer::new(DisplayMode::DEFAULT_1080P);

    // Clear background to dark navy blue (0x000F172A)
    unsafe {
        fb.draw_rect(0, 0, fb.mode.width, fb.mode.height, 0x000F_172A);
    }

    // Render Dual-Kernel GUI status bar
    ui::status_bar::draw(&mut fb);

    crate::arch::log("-----------------------------------------------------\n");
    crate::arch::log("[EggOS Dual-Kernel]: Security Domain GUI Active.\n");
    crate::arch::log("-----------------------------------------------------\n\n");

    crate::arch::log("[Self-Test]: Testing Capped Memory Stream...\n");
    let test_bytes = 150 * 1024;
    match allocate_with_capability(2, test_bytes, CapRights::READ_WRITE) {
        Ok((ptr, token)) => {
            crate::arch::log(" -> Memory Allocation & Token Minting: SUCCESS\n");

            match capability::GLOBAL_CAP_GATE.lock().verify_access(
                token.id,
                ptr,
                test_bytes,
                CapRights::READ_WRITE,
            ) {
                Ok(_) => crate::arch::log(" -> Capability Gate Access: GRANTED\n"),
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
    crate::arch::log(" [EggOS Dual-Kernel]: All subsystems operational.     \n");
    crate::arch::log("=====================================================\n");
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // Print location and reason to serial before infinite pause
    crate::arch::log("[SECURITY_KERNEL PANIC]\n");
    loop {
        #[cfg(target_arch = "x86_64")]
        core::arch::x86_64::_mm_pause();
    }
}

