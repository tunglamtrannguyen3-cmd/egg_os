pub mod battery;
pub mod keyboard;
pub mod vga;

// Re-exports for easy kernel usage
pub use battery::{BatteryInfo, BatteryStatus, PowerProfile, BatterySaver, BATTERY_SAVER};
pub use keyboard::Keyboard;
pub use vga::{Color, VgaWriter};

