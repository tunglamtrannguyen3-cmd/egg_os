#[derive(Debug, Clone, Copy)]
pub struct DisplayMode {
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub bpp: usize,
    pub base_addr: usize,
}

impl DisplayMode {
    pub const DEFAULT_1080P: Self = Self {
        width: 1920,
        height: 1080,
        pitch: 1920 * 4,
        bpp: 4,
        base_addr: 0xFD000000,
    };
}

pub struct Framebuffer {
    pub mode: DisplayMode,
    base_ptr: *mut u8,
}

impl Framebuffer {
    pub fn new(mode: DisplayMode) -> Self {
        Self {
            mode,
            base_ptr: mode.base_addr as *mut u8,
        }
    }

    pub unsafe fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.mode.width && y < self.mode.height {
            let byte_offset = y * self.mode.pitch + x * self.mode.bpp;
            let pixel_ptr = self.base_ptr.add(byte_offset) as *mut u32;
            pixel_ptr.write_volatile(color);
        }
    }

    pub unsafe fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        let max_x = core::cmp::min(x + w, self.mode.width);
        let max_y = core::cmp::min(y + h, self.mode.height);

        for row in y..max_y {
            for col in x..max_x {
                self.draw_pixel(col, row, color);
            }
        }
    }
}
