extern crate alloc;
use alloc::vec::Vec;
use uefi::cstr16;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::MemoryType;

pub fn load_kernel_file(
    _image_handle: Handle,
    system_table: &mut SystemTable<Boot>,
) -> Result<Vec<u8>, &'static str> {
    let boot_services = system_table.boot_services();

    let fs_handle = boot_services
        .get_handle_for_protocol::<SimpleFileSystem>()
        .map_err(|_| "Failed to get filesystem handle")?;

    let mut fs = boot_services
        .open_protocol_exclusive::<SimpleFileSystem>(fs_handle)
        .map_err(|_| "Failed to open filesystem protocol")?;

    let mut root = fs.open_volume().map_err(|_| "Failed to open volume")?;

    let mut file = root
        .open(cstr16!("boot\\host_kernel"), FileMode::Read, FileAttribute::empty())
        .map_err(|_| "Failed to open host_kernel")?
        .into_regular_file()
        .ok_or("Not a regular file")?;

    let mut info_buf = [0u8; 256];
    let info: &FileInfo = file
        .get_info(&mut info_buf)
        .map_err(|_| "Failed to get file info")?;
    let file_size = info.file_size() as usize;

    let buffer_ptr = boot_services
        .allocate_pages(
            uefi::table::boot::AllocateType::AnyPages,
            MemoryType::LOADER_DATA,
            (file_size + 4095) / 4096,
        )
        .map_err(|_| "Failed to allocate memory for kernel binary")?;

    let buffer = unsafe { core::slice::from_raw_parts_mut(buffer_ptr as *mut u8, file_size) };
    file.read(buffer).map_err(|_| "Failed to read file into buffer")?;

    Ok(buffer.to_vec())
}
