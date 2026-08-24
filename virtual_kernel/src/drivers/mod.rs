pub mod battery;
pub mod keyboard;
pub mod vga;

pub use battery::{
    check_battery_status, handle_key_input, BatteryInfo, BatterySaver, BatteryStatus,
    PowerProfile, BATTERY_SAVER,
};
pub use vga::{Color, VgaWriter};