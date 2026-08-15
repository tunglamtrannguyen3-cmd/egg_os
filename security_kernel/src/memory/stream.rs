// src/memory/stream.rs

pub const MAX_CHUNK_CAP: usize = 64 * 1024; // 64 KiB Hard Cap

/// Adaptive chunk stream generator.
/// - Returns exact remaining size if total <= 64 KiB.
/// - Iteratively yields 64 KiB slices until the remainder fits in the final window.
pub struct MatchChunkStream {
    remaining_bytes: usize,
}

impl MatchChunkStream {
    pub fn new(total_bytes: usize) -> Self {
        Self {
            remaining_bytes: total_bytes,
        }
    }

    pub fn remaining(&self) -> usize {
        self.remaining_bytes
    }
}

impl Iterator for MatchChunkStream {
    type Item = usize; // Returns byte size for the current chunk step

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_bytes == 0 {
            return None;
        }

        // Adaptive cap check: match exact size if under 64 KiB, otherwise cap at 64 KiB
        let chunk_size = core::cmp::min(self.remaining_bytes, MAX_CHUNK_CAP);
        self.remaining_bytes -= chunk_size;

        Some(chunk_size)
    }
}

/// Helper function to execute chunk transfers sequentially with exact offsets
pub fn process_data_stream<F>(total_bytes: usize, mut transfer_fn: F)
where
    F: FnMut(usize, usize) -> Result<(), &'static str>, // Closure arguments: (chunk_size, offset)
{
    let stream = MatchChunkStream::new(total_bytes);
    let mut offset = 0;

    for chunk_size in stream {
        if let Err(_err) = transfer_fn(chunk_size, offset) {
            crate::arch::log("[EggOS Memory Stream Error]: Transfer interrupted\n");
            break;
        }
        offset += chunk_size;
    }
}

