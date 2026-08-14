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

pub fn show_low_battery_dialog(framebuffer: &mut [u32], width: usize, height: usize) {
    crate::arch::log("[EggOS UI Alert]: Low Battery Warning Dialog Triggered\n");
    // TODO: Add pixel rendering logic for the dialog box here when ready
}
