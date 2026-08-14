// src/ui/mod.rs

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

pub fn show_low_battery_dialog(_framebuffer: &mut [u32], _width: usize, _height: usize) {
    crate::arch::log("\n=====================================================\n");
    crate::arch::log(" [WARNING]: Low Battery Detected! (<= 15% Remaining)  \n");
    crate::arch::log("-----------------------------------------------------\n");
    crate::arch::log(" Please choose an action:                             \n");
    crate::arch::log("   [1] Turn on Battery Saving                        \n");
    crate::arch::log("   [2] Plugged In / Charging                         \n");
    crate::arch::log("=====================================================\n\n");
}
