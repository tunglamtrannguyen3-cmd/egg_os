use super::framebuffer::Framebuffer;

pub const STATUS_BAR_HEIGHT: usize = 32;
pub const COLOR_BG: u32 = 0x001E_1E2E;
pub const COLOR_ACCENT: u32 = 0x0038_BDF8;
pub const COLOR_BATTERY: u32 = 0x00A6_E3A1;
pub const COLOR_SAVER: u32 = 0x00FA_B387;

pub struct StatusBar;

impl StatusBar {
    pub fn render(fb: &mut Framebuffer, battery_level: u8, power_saving: bool) {
        let screen_width = fb.mode.width;

        unsafe {
            // Render background bar
            fb.draw_rect(0, 0, screen_width, STATUS_BAR_HEIGHT, COLOR_BG);

            // Render accent border line directly beneath status bar
            fb.draw_rect(0, STATUS_BAR_HEIGHT, screen_width, 2, COLOR_ACCENT);

            // Render battery level indicator on the top right
            if screen_width >= 100 {
                let bat_width = (battery_level as usize) * 80 / 100;
                let bat_color = if power_saving {
                    COLOR_SAVER
                } else {
                    COLOR_BATTERY
                };
                fb.draw_rect(screen_width - 100, 8, bat_width, 16, bat_color);
            }
        }
    }
}

pub fn draw(fb: &mut Framebuffer) {
    // Default render pass called during boot (100% battery, normal power)
    StatusBar::render(fb, 100, false);
}

pub fn show_job_finished_toast(_app_name: &'static str) {
    // Toast notification render logic
}
