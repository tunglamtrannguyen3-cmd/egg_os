pub const PAGE_SIZE: usize = 4096;               // Hardware minimum (4 KiB)
pub const MAX_CHUNK_SIZE: usize = 64 * 1024;     // 64 KiB Cap (or 64 * 1024 * 1024 for 64 MiB)

/// Dynamically slices data: exact size if smaller than cap, or chunked if larger.
pub fn process_data_chunks<F>(data: &[u8], mut process_chunk: F) -> Result<(), &'static str>
where
    F: FnMut(usize, &[u8]) -> Result<(), &'static str>, // (chunk_index, slice)
{
    if data.is_empty() {
        return Ok(());
    }

    let mut offset = 0;
    let mut chunk_index = 0;

    while offset < data.len() {
        // 1. If remaining data < MAX_CHUNK_SIZE, take remaining exact bytes (e.g., 1 byte)
        // 2. If remaining data >= MAX_CHUNK_SIZE, take MAX_CHUNK_SIZE
        let bytes_to_process = usize::min(data.len() - offset, MAX_CHUNK_SIZE);
        
        let chunk_slice = &data[offset..offset + bytes_to_process];

        // Process current chunk
        process_chunk(chunk_index, chunk_slice)?;

        // Advance offset and chunk count
        offset += bytes_to_process;
        chunk_index += 1;
    }

    Ok(())
}
