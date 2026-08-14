pub mod framebuffer;
pub mod status_bar;

pub use framebuffer::{DisplayMode, Framebuffer};
pub use status_bar::StatusBar;

pub fn init() {
    crate::arch::log("[EggOS UI Subsystem: Dynamic Resolution Display Engine Online]\n");
}

pub fn render_desktop(mode: DisplayMode, battery_level: u8, power_saving: bool) {
    let mut fb = Framebuffer::new(mode);
    StatusBar::render(&mut fb, battery_level, power_saving);
}
