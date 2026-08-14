// src/drivers/battery/mod.rs

pub mod ec;
pub mod saver;
pub mod state;

use spin::Mutex;
pub use state::{BatteryInfo, BatteryStatus, PowerProfile};
pub use saver::{BatterySaver, PowerPromptState, PowerChoice};

pub static BATTERY_SAVER: Mutex<BatterySaver> = Mutex::new(BatterySaver::new());

pub fn init() {
    crate::arch::log("[EggOS Driver: x86_64 Battery Subsystem Online]\n");
}

/// Periodic battery polling function called by kernel loop or timer interrupt
pub fn poll(framebuffer: Option<&mut [u32]>, screen_width: usize, screen_height: usize) {
    // 1. Read battery telemetry directly from physical x86 EC
    let info = ec::EmbeddedController::read_status();
    
    // 2. Evaluate state and update saver profile/prompt state
    let mut saver = BATTERY_SAVER.lock();
    saver.update(&info);

    // 3. Check your rich prompt state from saver.rs
    if saver.prompt_state == PowerPromptState::AwaitingLowBatteryChoice {
        if let Some(fb) = framebuffer {
            crate::ui::show_low_battery_dialog(fb, screen_width, screen_height);
        }
    }
}

/// Routes user keypresses from PS/2 keyboard to battery power profile manager
pub fn handle_input(key: char) {
    let mut saver = BATTERY_SAVER.lock();
    // Leverages your built-in handle_key_input method directly!
    saver.handle_key_input(key);
}
