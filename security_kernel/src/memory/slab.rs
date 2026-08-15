// src/memory/slab.rs

use super::frame::{CHUNK_ALLOCATOR, CHUNK_SIZE};
use spin::Mutex;

/// Sub-chunk allocator for tight allocations under 64 KiB
pub struct SlabAllocator {
    current_chunk_ptr: usize,
    chunk_offset: usize,
}

impl SlabAllocator {
    pub const fn new() -> Self {
        Self {
            current_chunk_ptr: 0,
            chunk_offset: 0,
        }
    }

    /// Packs small object allocations into 8-byte aligned sub-blocks inside a 64 KiB frame
    pub fn allocate_small(&mut self, size: usize) -> Option<usize> {
        if size == 0 || size >= CHUNK_SIZE {
            return None;
        }

        // Align request size to 8-byte boundaries safely
        let aligned_size = size.checked_add(7)? & !7;

        // If active 64 KiB chunk is missing or full, grab a fresh chunk from frame manager
        let required_end = self.chunk_offset.checked_add(aligned_size)?;
        if self.current_chunk_ptr == 0 || required_end > CHUNK_SIZE {
            let new_chunk = CHUNK_ALLOCATOR.lock().allocate_chunk()?;
            self.current_chunk_ptr = new_chunk;
            self.chunk_offset = 0;
        }

        let allocated_ptr = self.current_chunk_ptr.checked_add(self.chunk_offset)?;
        self.chunk_offset += aligned_size;
        Some(allocated_ptr)
    }
}

pub static SLAB_ALLOCATOR: Mutex<SlabAllocator> = Mutex::new(SlabAllocator::new());

