#![no_std]

/// Reads a single raw byte from x86 I/O Port 0x60 (PS/2 Data Port)
pub fn read_io_port_60() -> u8 {
    let scancode: u8;
    unsafe {
        core::arch::asm!("in al, dx", out("al") scancode, in("dx") 0x60u16);
    }
    scancode
}

