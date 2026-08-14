pub mod task;

pub use task::{TaskControlBlock, TaskStatus};

pub fn init() {
    crate::arch::log("[EggOS Scheduler: Process Freeze Counter & Thread Scaling Ready]\n");
}
