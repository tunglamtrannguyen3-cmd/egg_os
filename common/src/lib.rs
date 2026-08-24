#![no_std]
#![no_main]
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
    pub virtual_kernel_addr: u64,
    pub virtual_kernel_size: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ModuleInfo {
    pub base_address: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ModulesResponse {
    pub module_count: usize,
    pub modules: *const ModuleInfo,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ModulesRequest {
    pub response: Option<ModulesResponse>,
}

impl ModulesRequest {
    pub const fn new() -> Self {
        Self { response: None }
    }

    pub fn response(&self) -> Option<&ModulesResponse> {
        self.response.as_ref()
    }
}
// Put these at the bottom of common/src/lib.rs
unsafe impl Sync for ModulesRequest {}
unsafe impl Send for ModulesRequest {}
unsafe impl Sync for ModulesResponse {}
unsafe impl Send for ModulesResponse {}
