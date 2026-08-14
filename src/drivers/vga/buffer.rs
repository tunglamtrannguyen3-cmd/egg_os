#![no_std]

use super::colors::{Color, ColorCode};

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;
const VGA_ADDRESS: usize = 0xb8000;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

pub struct VgaWriter {
    column_position: usize,
    color_code: ColorCode,
}

impl VgaWriter {
    pub const fn new(foreground: Color, background: Color) -> Self {
        Self {
            column_position: 0,
            color_code: ColorCode::new(foreground, background),
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let buffer_ptr = VGA_ADDRESS as *mut ScreenChar;
                let index = row * BUFFER_WIDTH + col;

                unsafe {
                    buffer_ptr.add(index).write_volatile(ScreenChar {
                        ascii_character: byte,
                        color_code: self.color_code,
                    });
                }

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe), // Render unknown bytes as a block
            }
        }
    }

    fn new_line(&mut self) {
        // TODO: Add vertical buffer shifting/scrolling here
        self.column_position = 0;
    }
}

