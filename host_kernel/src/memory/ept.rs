use spin::Mutex;

pub static HOST_PHYSICAL_ALLOCATOR: Mutex<HostMemoryManager> = Mutex::new(HostMemoryManager::new());

pub struct HostMemoryManager {
    next_free_frame: u64,
}

impl HostMemoryManager {
    pub const fn new() -> Self {
        // Reserve lower physical addresses and start allocations above 16MB
        Self { next_free_frame: 0x1000000 }
    }

    pub fn allocate_frame(&mut self) -> u64 {
        let frame = self.next_free_frame;
        self.next_free_frame += 4096; // 4KB frame
        frame
    }
}

pub fn init_host_memory() {
    // Initialize Host EPT PML4 table structures here
}

