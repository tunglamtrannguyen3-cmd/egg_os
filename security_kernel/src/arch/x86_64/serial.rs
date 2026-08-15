// src/arch/x86_64/serial.rs

use spin::Mutex;

pub struct SerialPort {
    port: u16,
}

impl SerialPort {
    pub const fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn init(&self) {
        unsafe {
            outb(self.port + 1, 0x00); // Disable interrupts
            outb(self.port + 3, 0x80); // Enable DLAB (set baud rate divisor)
            outb(self.port + 0, 0x03); // Set divisor to 3 (38400 baud) lo byte
            outb(self.port + 1, 0x00); // hi byte
            outb(self.port + 3, 0x03); // 8 bits, no parity, one stop bit
            outb(self.port + 2, 0xC7); // Enable FIFO, clear, 14-byte threshold
            outb(self.port + 4, 0x0B); // IRQs enabled, RTS/DSR set
        }
    }

    pub fn write_byte(&self, byte: u8) {
        unsafe {
            while (inb(self.port + 5) & 0x20) == 0 {}
            outb(self.port, byte);
        }
    }

    pub fn write_str(&self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}

#[inline]
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    core::arch::asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags));
    ret
}

pub static SERIAL1: Mutex<SerialPort> = Mutex::new(SerialPort::new(0x3F8));

pub fn init() {
    SERIAL1.lock().init();
}

pub fn print_str(s: &str) {
    SERIAL1.lock().write_str(s);
}

