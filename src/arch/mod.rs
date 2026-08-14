// src/arch/mod.rs

pub mod serial;

/// Low-level x86 port output instruction
pub unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val);
}

/// Low-level x86 port input instruction
pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    core::arch::asm!("in al, dx", in("dx") port, out("al") ret);
    ret
}

/// Writes debug messages directly to COM1 Serial Port
pub fn log(msg: &str) {
    for byte in msg.bytes() {
        unsafe {
            outb(0x3F8, byte);
        }
    }
}

