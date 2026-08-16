use super::framebuffer::Framebuffer;

/// Draw a 1-pixel line using Bresenham's algorithm
pub fn draw_line(fb: &mut Framebuffer, mut x0: usize, mut y0: usize, x1: usize, y1: usize, color: u32) {
    let dx = (x1 as isize - x0 as isize).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 as isize - y0 as isize).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        unsafe { fb.draw_pixel(x0, y0, color); }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 = (x0 as isize + sx) as usize;
        }
        if e2 <= dx {
            err += dx;
            y0 = (y0 as isize + sy) as usize;
        }
    }
}

/// Draw an unfilled circle
pub fn draw_circle(fb: &mut Framebuffer, x0: usize, y0: usize, radius: usize, color: u32) {
    let mut x = radius as isize;
    let mut y = 0isize;
    let mut err = 0isize;

    while x >= y {
        unsafe {
            fb.draw_pixel((x0 as isize + x) as usize, (y0 as isize + y) as usize, color);
            fb.draw_pixel((x0 as isize + y) as usize, (y0 as isize + x) as usize, color);
            fb.draw_pixel((x0 as isize - y) as usize, (y0 as isize + x) as usize, color);
            fb.draw_pixel((x0 as isize - x) as usize, (y0 as isize + y) as usize, color);
            fb.draw_pixel((x0 as isize - x) as usize, (y0 as isize - y) as usize, color);
            fb.draw_pixel((x0 as isize - y) as usize, (y0 as isize - x) as usize, color);
            fb.draw_pixel((x0 as isize + y) as usize, (y0 as isize - x) as usize, color);
            fb.draw_pixel((x0 as isize + x) as usize, (y0 as isize - y) as usize, color);
        }

        if err <= 0 {
            y += 1;
            err += 2 * y + 1;
        }
        if err > 0 {
            x -= 1;
            err -= 2 * x + 1;
        }
    }
}
