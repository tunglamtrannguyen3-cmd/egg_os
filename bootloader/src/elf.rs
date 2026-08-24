use core::ptr;
use uefi::boot::{self, AllocateType, MemoryType};

const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
const PT_LOAD: u32 = 1;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Elf64Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

pub struct LoadedElf {
    pub entry_point: u64,
}

pub fn parse_and_map(buffer: &[u8]) -> Result<LoadedElf, &'static str> {
    if buffer.len() < core::mem::size_of::<Elf64Header>() {
        return Err("Buffer too small for ELF header");
    }

    let header_ptr = buffer.as_ptr() as *const Elf64Header;
    
    // Safety: Read header safely using unaligned reads to avoid UB on packed structs
    let e_ident = unsafe { ptr::addr_of!((*header_ptr).e_ident).read_unaligned() };
    if &e_ident[0..4] != ELF_MAGIC {
        return Err("Invalid ELF magic number");
    }

    let e_entry = unsafe { ptr::addr_of!((*header_ptr).e_entry).read_unaligned() };
    let phoff = unsafe { ptr::addr_of!((*header_ptr).e_phoff).read_unaligned() } as usize;
    let phentsize = unsafe { ptr::addr_of!((*header_ptr).e_phentsize).read_unaligned() } as usize;
    let phnum = unsafe { ptr::addr_of!((*header_ptr).e_phnum).read_unaligned() } as usize;

    for i in 0..phnum {
        let offset = phoff + i * phentsize;
        if offset + core::mem::size_of::<Elf64Phdr>() > buffer.len() {
            return Err("Program header out of bounds");
        }

        let phdr_ptr = unsafe { buffer.as_ptr().add(offset) as *const Elf64Phdr };
        let p_type = unsafe { ptr::addr_of!((*phdr_ptr).p_type).read_unaligned() };

        if p_type == PT_LOAD {
            let p_vaddr = unsafe { ptr::addr_of!((*phdr_ptr).p_vaddr).read_unaligned() };
            let p_offset = unsafe { ptr::addr_of!((*phdr_ptr).p_offset).read_unaligned() } as usize;
            let filesz = unsafe { ptr::addr_of!((*phdr_ptr).p_filesz).read_unaligned() } as usize;
            let memsz = unsafe { ptr::addr_of!((*phdr_ptr).p_memsz).read_unaligned() } as usize;

            // Convert higher-half virtual address (0xFFFFFFFF80100000) to physical address offset
            // Assuming higher-half mapping where 0xFFFFFFFF80000000 maps to Physical 0x0
            let phys_addr = if p_vaddr >= 0xffff_ffff_8000_0000 {
                p_vaddr - 0xffff_ffff_8000_0000
            } else {
                p_vaddr
            };

            let pages = (memsz + 4095) / 4096;
            
            // Allocate physical pages using UEFI Boot Services
            // Allocate physical pages using UEFI Boot Services
            let dest_ptr: *mut u8 = if phys_addr != 0 {
                let _ = boot::allocate_pages(
                    AllocateType::Address(phys_addr),
                    MemoryType::LOADER_DATA,
                    pages,
                );
                phys_addr as *mut u8
            } else {
                let ptr = boot::allocate_pages(
                    AllocateType::AnyPages,
                    MemoryType::LOADER_DATA,
                    pages,
                ).map_err(|_| "Failed to allocate physical memory for ELF segment")?;
                
                ptr.as_ptr()
            };
            unsafe {
                if filesz > 0 {
                    ptr::copy_nonoverlapping(buffer.as_ptr().add(p_offset), dest_ptr, filesz);
                }
                if memsz > filesz {
                    ptr::write_bytes(dest_ptr.add(filesz), 0, memsz - filesz);
                }
            }
        }
    }

    Ok(LoadedElf {
        entry_point: e_entry,
    })
}