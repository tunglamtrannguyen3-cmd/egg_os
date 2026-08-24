use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryInfo {
    pub percentage: u8,
    pub status: BatteryStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerPromptState {
    Normal,
    AwaitingLowBatteryAck,
    Dismissed,
}

pub struct BatterySaver {
    pub prompt_state: PowerPromptState,
    pub battery_percent: u8,
    pub is_charging: bool,
    pub profile: PowerProfile,
}

impl BatterySaver {
    pub const fn new() -> Self {
        Self {
            prompt_state: PowerPromptState::AwaitingLowBatteryAck,
            battery_percent: 15,
            is_charging: false,
            profile: PowerProfile::PowerSaver,
        }
    }

    pub fn handle_key_input(&mut self, key: char) {
        if key == '\n' || key == '\r' || key == '\x1B' {
            self.prompt_state = PowerPromptState::Dismissed;
        }
    }
}

pub struct Mutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        while self.lock.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        MutexGuard { mutex: self }
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> core::ops::Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.lock.store(false, Ordering::Release);
    }
}

pub static BATTERY_SAVER: Mutex<BatterySaver> = Mutex::new(BatterySaver::new());
