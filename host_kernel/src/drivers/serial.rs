use core::arch::asm;
use core::fmt;

const COM1: u16 = 0x3F8;

#[inline]
unsafe fn outb(port: u16, val: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
        options(nomem, nostack, preserves_flags)
    );
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    asm!(
        "in al, dx",
        in("dx") port,
        out("al") val,
        options(nomem, nostack, preserves_flags)
    );
    val
}

pub struct SerialPort;

impl SerialPort {
    pub fn init() {
        unsafe {
            outb(COM1 + 1, 0x00); // Disable interrupts
            outb(COM1 + 3, 0x80); // Enable DLAB (set baud rate divisor)
            outb(COM1 + 0, 0x03); // Set baud rate divisor to 3 (38400 baud)
            outb(COM1 + 1, 0x00);
            outb(COM1 + 3, 0x03); // 8 bits, no parity, one stop bit
            outb(COM1 + 2, 0xC7); // Enable FIFO, clear, 14-byte threshold
            outb(COM1 + 4, 0x0B); // IRQs enabled, RTS/DSR set
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            while (inb(COM1 + 5) & 0x20) == 0 {}
            outb(COM1, byte);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::drivers::serial::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut serial = SerialPort;
    let _ = serial.write_fmt(args);
}