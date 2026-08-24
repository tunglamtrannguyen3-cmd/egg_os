// src/drivers/battery/saver.rs

use super::state::{BatteryInfo, BatteryStatus, PowerProfile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerPromptState {
    None,
    AwaitingLowBatteryChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerChoice {
    EnableSaver,
    PluggedInCharging,
}

pub struct BatterySaver {
    pub enabled: bool,
    pub auto_enable: bool,
    pub threshold_percentage: u8,
    pub active_profile: PowerProfile,
    pub prompt_state: PowerPromptState,
}

impl BatterySaver {
    pub const fn new() -> Self {
        Self {
            enabled: false,
            auto_enable: true,
            threshold_percentage: 15,
            active_profile: PowerProfile::Balanced,
            prompt_state: PowerPromptState::None,
        }
    }

    pub fn update(&mut self, info: &BatteryInfo) -> PowerProfile {
        match info.status {
            BatteryStatus::Charging | BatteryStatus::Full => {
                self.enabled = false;
                self.prompt_state = PowerPromptState::None;
                self.active_profile = PowerProfile::Performance;
            }
            BatteryStatus::Discharging => {
                if info.percentage <= self.threshold_percentage {
                    if !self.enabled && self.prompt_state == PowerPromptState::None {
                        self.prompt_state = PowerPromptState::AwaitingLowBatteryChoice;
                    }
                } else {
                    self.prompt_state = PowerPromptState::None;
                    if !self.enabled {
                        self.active_profile = PowerProfile::Balanced;
                    }
                }
            }
            BatteryStatus::NotPresent | BatteryStatus::Unknown => {
                self.active_profile = PowerProfile::Balanced;
            }
        }
        self.active_profile
    }

    pub fn handle_key_input(&mut self, key: char) -> Option<PowerProfile> {
        if self.prompt_state != PowerPromptState::AwaitingLowBatteryChoice {
            return None;
        }

        match key {
            '1' => Some(self.apply_choice(PowerChoice::EnableSaver)),
            '2' => Some(self.apply_choice(PowerChoice::PluggedInCharging)),
            _ => None,
        }
    }

    pub fn apply_choice(&mut self, choice: PowerChoice) -> PowerProfile {
        self.prompt_state = PowerPromptState::None;

        match choice {
            PowerChoice::EnableSaver => {
                self.enabled = true;
                self.active_profile = PowerProfile::PowerSaver;
                crate::arch::log("[EggOS Power]: Battery Saver Profile Active.\n");
            }
            PowerChoice::PluggedInCharging => {
                self.enabled = false;
                self.active_profile = PowerProfile::Performance;
                crate::arch::log("[EggOS Power]: Charger connected! Performance Boost Active!\n");
            }
        }
        self.active_profile
    }
}
