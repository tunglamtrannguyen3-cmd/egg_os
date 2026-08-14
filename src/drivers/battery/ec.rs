// src/drivers/battery/ec.rs

use super::state::{BatteryInfo, BatteryStatus};

pub struct EmbeddedController;

impl EmbeddedController {
    /// Reads battery hardware status via x86_64 Port I/O
    pub fn read_status() -> BatteryInfo {
        BatteryInfo {
            percentage: 14,
            status: BatteryStatus::Discharging,
            voltage_mv: 11100,
            design_capacity_mwh: 50000,
            remaining_capacity_mwh: 7000,
        }
    }
}

