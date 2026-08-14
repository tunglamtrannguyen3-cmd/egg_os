// src/drivers/battery/mod.rs

pub mod ec;
pub mod saver;
pub mod state;

use spin::Mutex;
pub use state::{BatteryInfo, BatteryStatus, PowerProfile};
pub use saver::BatterySaver;

pub static BATTERY_SAVER: Mutex<BatterySaver> = Mutex::new(BatterySaver::new());

pub fn init() {
    crate::arch::log("[EggOS Driver: x86_64 Battery Subsystem Online]\n");
}

/// Periodic battery polling function called by kernel loop or timer interrupt
pub fn poll(framebuffer: Option<&mut [u32]>, screen_width: usize, screen_height: usize) {
    // 1. Read battery telemetry directly from physical x86 EC
    let info = ec::EmbeddedController::read_status();
    
    // 2. Evaluate state and trigger low battery alert if needed
    let mut saver = BATTERY_SAVER.lock();
    saver.update(&info);

    if saver.should_show_dialog() {
        if let Some(fb) = framebuffer {
            crate::ui::show_low_battery_dialog(fb, screen_width, screen_height);
        }
    }
}

/// Routes user keypresses from PS/2 keyboard to battery power profile manager
pub fn handle_input(key: char) {
    let mut saver = BATTERY_SAVER.lock();
    match key {
        '1' => {
            saver.set_profile(PowerProfile::PowerSaver);
            crate::arch::log("[EggOS Power Management]: Power Saver Profile Active\n");
        }
        '2' => {
            // FIXED: Changed HighPerformance to Performance to match PowerProfile enum definition
            saver.set_profile(PowerProfile::Performance);
            crate::arch::log("[EggOS Power Management]: High Performance Profile Active\n");
        }
        _ => {}
    }
}
