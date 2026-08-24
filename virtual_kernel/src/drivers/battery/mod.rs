pub mod state;

use crate::ui::show_low_battery_dialog;
use crate::ui::Framebuffer;

pub use state::{
    BatteryInfo, BatterySaver, BatteryStatus, PowerProfile, PowerPromptState, BATTERY_SAVER,
};

pub fn check_battery_status(fb: &mut Framebuffer, screen_width: usize, screen_height: usize) {
    let saver = BATTERY_SAVER.lock();
    if saver.prompt_state == PowerPromptState::AwaitingLowBatteryAck {
        show_low_battery_dialog(fb, screen_width, screen_height);
    }
}

pub fn handle_key_input(key: char) {
    let mut saver = BATTERY_SAVER.lock();
    saver.handle_key_input(key);
}