// src/memory/frame.rs

use spin::Mutex;

/// Base frame granule: 64 KiB (65,536 bytes)
pub const CHUNK_SIZE: usize = 64 * 1024;

pub struct ChunkAllocator {
    next_chunk: usize,
    memory_end: usize,
    allocated_chunks: usize,
}

impl ChunkAllocator {
    pub const fn new() -> Self {
        Self {
            next_chunk: 0,
            memory_end: 0,
            allocated_chunks: 0,
        }
    }

    pub fn init(&mut self, start_addr: usize, size: usize) {
        // Align starting RAM address to 64 KiB boundary safely
        let aligned_start = start_addr.checked_add(CHUNK_SIZE - 1)
            .map(|val| val & !(CHUNK_SIZE - 1))
            .unwrap_or(start_addr);

        self.next_chunk = aligned_start;
        self.memory_end = start_addr.saturating_add(size);
        self.allocated_chunks = 0;
    }

    /// Hand out a single 64 KiB frame
    pub fn allocate_chunk(&mut self) -> Option<usize> {
        let end_ptr = self.next_chunk.checked_add(CHUNK_SIZE)?;
        if end_ptr <= self.memory_end {
            let ptr = self.next_chunk;
            self.next_chunk = end_ptr;
            self.allocated_chunks += 1;
            Some(ptr)
        } else {
            None
        }
    }

    /// Stream multiple contiguous 64 KiB chunks for large allocations (>= 64 KiB)
    pub fn allocate_bytes(&mut self, bytes: usize) -> Option<usize> {
        if bytes == 0 {
            return None;
        }

        // Prevent overflow during chunk count calculation
        let padded_bytes = bytes.checked_add(CHUNK_SIZE - 1)?;
        let chunks_needed = padded_bytes / CHUNK_SIZE;
        let total_bytes = chunks_needed.checked_mul(CHUNK_SIZE)?;

        let end_ptr = self.next_chunk.checked_add(total_bytes)?;
        if end_ptr <= self.memory_end {
            let start_ptr = self.next_chunk;
            self.next_chunk = end_ptr;
            self.allocated_chunks += chunks_needed;
            Some(start_ptr)
        } else {
            None // Physical RAM exhausted
        }
    }
}

pub static CHUNK_ALLOCATOR: Mutex<ChunkAllocator> = Mutex::new(ChunkAllocator::new());

pub fn init(start_ram: usize, ram_size: usize) {
    CHUNK_ALLOCATOR.lock().init(start_ram, ram_size);
}

