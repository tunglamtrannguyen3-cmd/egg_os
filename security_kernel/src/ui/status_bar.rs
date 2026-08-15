use super::framebuffer::Framebuffer;

pub const STATUS_BAR_HEIGHT: usize = 32;
pub const COLOR_BG: u32 = 0x001E1E2E;
pub const COLOR_BATTERY: u32 = 0x00A6E3A1;
pub const COLOR_SAVER: u32 = 0x00FAB387;

pub struct StatusBar;

impl StatusBar {
    pub fn render(fb: &mut Framebuffer, battery_level: u8, power_saving: bool) {
        let screen_width = fb.mode.width;

        unsafe {
            fb.draw_rect(0, 0, screen_width, STATUS_BAR_HEIGHT, COLOR_BG);
            let bat_width = (battery_level as usize) * 80 / 100;
            let bat_color = if power_saving { COLOR_SAVER } else { COLOR_BATTERY };
            fb.draw_rect(screen_width - 100, 8, bat_width, 16, bat_color);
        }
    }
}

pub fn show_job_finished_toast(app_name: &'static str) {
    let _ = app_name; // Render status bar toast logic here
}
