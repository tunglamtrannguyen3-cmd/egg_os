pub mod channel;

pub use channel::{IpcChannel, IpcMessage};

pub fn init() {
    crate::arch::log("[EggOS IPC Engine: Capability-Guarded Channels Online]\n");
}
