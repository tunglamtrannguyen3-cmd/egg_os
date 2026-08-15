pub mod frame;
pub mod slab;
pub mod state;
pub mod stream;

pub use frame::CHUNK_SIZE;
pub use state::{MemoryRegionState, ProcessMemState};
pub use stream::MatchChunkStream;

pub fn init() {
    frame::init(0x0080_0000, 32 * 1024 * 1024);
    crate::arch::log("[EggOS Memory Subsystem: Adaptive Matching + Capped Streamer Online]\n");
}

pub fn allocate(size: usize) -> Option<usize> {
    if size < frame::CHUNK_SIZE {
        slab::SLAB_ALLOCATOR.lock().allocate_small(size)
    } else {
        frame::CHUNK_ALLOCATOR.lock().allocate_bytes(size)
    }
}
