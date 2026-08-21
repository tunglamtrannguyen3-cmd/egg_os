extern crate alloc;
use alloc::vec::Vec;
use uefi::boot::{self, AllocateType, MemoryType};
use uefi::cstr16;
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;

pub fn load_kernel_file(
    _image_handle: uefi::Handle,
) -> Result<Vec<u8>, &'static str> {
    // 1. Locate the file system handle using modern boot services
    let fs_handle = boot::get_handle_for_protocol::<SimpleFileSystem>()
        .map_err(|_| "Failed to get filesystem handle")?;

    // 2. Open the SimpleFileSystem protocol exclusively
    let mut fs = boot::open_protocol_exclusive::<SimpleFileSystem>(fs_handle)
        .map_err(|_| "Failed to open filesystem protocol")?;

    // 3. Open root volume
    let mut root = fs.open_volume().map_err(|_| "Failed to open volume")?;

    // 4. Open host kernel binary
    let mut file = root
        .open(
            cstr16!("host_kernel"),
            FileMode::Read,
            FileAttribute::empty(),
        )
        .map_err(|_| "Failed to open host_kernel")?
        .into_regular_file()
        .ok_or("Not a regular file")?;

    // 5. Query kernel binary file size
    let mut info_buf = [0u8; 256];
    let info: &FileInfo = file
        .get_info(&mut info_buf)
        .map_err(|_| "Failed to get file info")?;
    let file_size = info.file_size() as usize;

    // 6. Calculate required 4KiB page count
    let pages = (file_size + 4095) / 4096;

    // 7. Allocate page-aligned memory buffer
    let non_null_ptr = boot::allocate_pages(
        AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        pages,
    )
    .map_err(|_| "Failed to allocate memory for kernel binary")?;

    // 8. Convert NonNull<u8> to a raw *mut u8 via .as_ptr()
    let raw_ptr = non_null_ptr.as_ptr();
    let buffer = unsafe { core::slice::from_raw_parts_mut(raw_ptr, file_size) };
    file.read(buffer).map_err(|_| "Failed to read file into buffer")?;

    Ok(buffer.to_vec())
}