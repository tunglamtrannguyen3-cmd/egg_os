use crate::memory::ProcessMemState;
use crate::capability::CapToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Ready,
    Running,
    Frozen,
    Terminated,
}

pub struct TaskControlBlock {
    pub id: u64,
    pub owner_tier: u8,
    pub status: TaskStatus,
    pub idle_time_seconds: u64,
    pub cap_token: Option<CapToken>,
}

impl TaskControlBlock {
    pub fn new(id: u64, owner_tier: u8) -> Self {
        Self {
            id,
            owner_tier,
            status: TaskStatus::Ready,
            idle_time_seconds: 0,
            cap_token: None,
        }
    }

    /// Increments idle clock and triggers freeze if background task hits 300 seconds (5 mins)
    pub fn tick_idle(&mut self) -> bool {
        if self.status == TaskStatus::Ready || self.status == TaskStatus::Running {
            self.idle_time_seconds += 1;
            if self.idle_time_seconds >= 300 {
                self.status = TaskStatus::Frozen;
                return true; // Indicates task transitioned to Frozen
            }
        }
        false
    }

    pub fn wake(&mut self) {
        self.status = TaskStatus::Ready;
        self.idle_time_seconds = 0;
    }
}
