pub mod buffer;
pub mod colors;

use core::fmt;
pub use colors::{Color, ColorCode};

pub struct VgaWriter;

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        crate::arch::log(s);
        Ok(())
    }
}

pub fn print_args(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = VgaWriter;
    let _ = writer.write_fmt(args);
}

#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => {
        $crate::drivers::vga::print_args(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! vga_println {
    () => {
        $crate::vga_print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::vga_print!("{}\n", format_args!($($arg)*))
    };
}
