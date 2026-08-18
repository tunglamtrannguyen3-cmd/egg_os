use limine::request::MemmapRequest;
use spin::Mutex;

pub const MAX_CHUNK_SIZE: usize = 64 * 1024; // 64 KiB cap

#[used]
#[unsafe(link_section = ".requests")]
static MEMORY_MAP_REQUEST: MemmapRequest = MemmapRequest::new();

pub static HOST_PHYSICAL_ALLOCATOR: Mutex<HostMemoryManager> = Mutex::new(HostMemoryManager::new());
pub static BUMP_ALLOCATOR: Mutex<BumpAllocator> = Mutex::new(BumpAllocator::new());

pub struct HostMemoryManager {
    next_alloc_ptr: u64,
    current_range_end: u64,
}

impl HostMemoryManager {
    pub const fn new() -> Self {
        Self {
            next_alloc_ptr: 0,
            current_range_end: 0,
        }
    }

    pub fn allocate_frame(&mut self) -> u64 {
        if self.next_alloc_ptr < self.current_range_end {
            let addr = self.next_alloc_ptr;
            self.next_alloc_ptr += 4096;
            return addr;
        }

        if let Some(resp) = MEMORY_MAP_REQUEST.response() {
            for entry in resp.entries().iter() {
                let entry_ptr = *entry as *const _ as *const u64;
                let base = unsafe { *entry_ptr.add(0) };
                let len = unsafe { *entry_ptr.add(1) };
                let mem_type = unsafe { *entry_ptr.add(2) };

                if mem_type == 0 && base > self.next_alloc_ptr {
                    self.next_alloc_ptr = (base + 4095) & !4095;
                    self.current_range_end = base + len;

                    let addr = self.next_alloc_ptr;
                    self.next_alloc_ptr += 4096;
                    return addr;
                }
            }
        }

        0
    }
}

// Sub-allocates exact byte sizes inside 4 KiB physical frames
pub struct BumpAllocator {
    current_frame: u64,
    offset: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            current_frame: 0,
            offset: 4096, // Force frame fetch on initial call
        }
    }

    pub fn alloc(&mut self, size: usize, align: usize) -> u64 {
        if size == 0 || size > 4096 {
            return 0;
        }

        let align_mask = align - 1;
        let aligned_offset = (self.offset + align_mask) & !align_mask;

        if aligned_offset + size > 4096 || self.current_frame == 0 {
            let mut manager = HOST_PHYSICAL_ALLOCATOR.lock();
            let new_frame = manager.allocate_frame();
            if new_frame == 0 {
                return 0;
            }
            self.current_frame = new_frame;
            self.offset = size;
            self.current_frame
        } else {
            let allocated_addr = self.current_frame + aligned_offset as u64;
            self.offset = aligned_offset + size;
            allocated_addr
        }
    }
}

pub fn process_data_chunks<F>(data: &[u8], mut process_fn: F)
where
    F: FnMut(&[u8]),
{
    for chunk in data.chunks(MAX_CHUNK_SIZE) {
        process_fn(chunk);
    }
}

pub fn init_host_memory() {
    let mut bump = BUMP_ALLOCATOR.lock();
    
    // Allocate exact 3 bytes aligned to 1-byte boundary
    let _three_byte_addr = bump.alloc(3, 1);

    let sample_data = [0u8; 64];
    process_data_chunks(&sample_data, |_chunk| {});
}
