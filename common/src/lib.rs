#![no_std]

pub const EGGOS_MAGIC: u64 = 0x4547475f4f535f21; // ASCII "EGG_OS_!"

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryMapEntry {
    pub phys_addr: u64,
    pub page_count: u64,
    pub entry_type: u32,
    pub _padding: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BootInfo {
    pub magic: u64,
    pub memory_map_ptr: *const MemoryMapEntry,
    pub memory_map_len: usize,
    pub security_kernel_addr: u64,
    pub security_kernel_size: u64,
}

