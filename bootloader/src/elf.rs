use core::ptr;

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
    let header = unsafe { &*header_ptr };

    if header.e_ident[0..4] != ELF_MAGIC {
        return Err("Invalid ELF magic number");
    }

    let phoff = header.e_phoff as usize;
    let phentsize = header.e_phentsize as usize;
    let phnum = header.e_phnum as usize;

    for i in 0..phnum {
        let offset = phoff + i * phentsize;
        if offset + core::mem::size_of::<Elf64Phdr>() > buffer.len() {
            return Err("Program header out of bounds");
        }

        let phdr_ptr = unsafe { buffer.as_ptr().add(offset) as *const Elf64Phdr };
        let phdr = unsafe { &*phdr_ptr };

        if phdr.p_type == PT_LOAD {
            let dest = phdr.p_paddr as *mut u8;
            let src_offset = phdr.p_offset as usize;
            let filesz = phdr.p_filesz as usize;
            let memsz = phdr.p_memsz as usize;

            unsafe {
                if filesz > 0 {
                    ptr::copy_nonoverlapping(buffer.as_ptr().add(src_offset), dest, filesz);
                }
                if memsz > filesz {
                    ptr::write_bytes(dest.add(filesz), 0, memsz - filesz);
                }
            }
        }
    }

    Ok(LoadedElf {
        entry_point: header.e_entry,
    })
}
