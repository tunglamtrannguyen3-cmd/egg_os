

pub fn allocate_kernel_stack() -> Result<u64, &'static str> {
    Ok(0x0009_0000)
}

pub fn build_boot_info() -> Result<*const common::BootInfo, &'static str> {
    Ok(core::ptr::null())
}