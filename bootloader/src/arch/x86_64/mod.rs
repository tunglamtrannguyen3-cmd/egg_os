use core::arch::global_asm;

// Embed the handoff assembly file directly into compilation
global_asm!(include_str!("switch.S"));

extern "C" {
    /// Transfers CPU execution from the bootloader to host_kernel.
    ///
    /// # Safety
    /// Interrupts are disabled, stack is switched, and this function never returns.
    pub fn jump_to_kernel(
        kernel_entry: u64,
        new_stack: u64,
        magic: u64,
        boot_info_ptr: *const common::BootInfo,
    ) -> !;
}


