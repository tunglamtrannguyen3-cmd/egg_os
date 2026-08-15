// src/drivers/battery/state.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
    NotPresent,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct BatteryInfo {
    pub percentage: u8,
    pub status: BatteryStatus,
    pub voltage_mv: u32,
    pub design_capacity_mwh: u32,
    pub remaining_capacity_mwh: u32,
}

impl BatteryInfo {
    pub const fn empty() -> Self {
        Self {
            percentage: 0,
            status: BatteryStatus::Unknown,
            voltage_mv: 0,
            design_capacity_mwh: 0,
            remaining_capacity_mwh: 0,
        }
    }
}

