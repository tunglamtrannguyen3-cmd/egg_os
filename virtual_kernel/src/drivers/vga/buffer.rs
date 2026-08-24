use super::colors::ColorCode;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Software VGA Buffer manager
pub struct VgaBufferManager {
    pub column_position: usize,
    pub color_code: ColorCode,
}

impl VgaBufferManager {
    pub const fn new(color_code: ColorCode) -> Self {
        Self {
            column_position: 0,
            color_code,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        self.column_position = 0;
    }
}
