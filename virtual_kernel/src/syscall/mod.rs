pub mod handler;

pub use handler::handle_syscall;

pub fn init() {
    crate::arch::log("[EggOS Syscall Gateway: Ring 2 User Intercepts Online]\n");
}
