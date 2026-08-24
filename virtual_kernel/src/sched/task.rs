// security_kernel/src/sched/task.rs

use crate::capability::CapToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Ready,
    Running,
    Frozen,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundTier {
    /// Privileged background execution (AI training, audio playback, games, daemons)
    /// Never subject to auto-freeze.
    PrivilegedBackground,
    /// Standard interactive application.
    /// Freezes after 300 seconds of inactivity when active work finishes.
    Standard,
}

pub struct TaskControlBlock {
    pub id: u64,
    pub name: &'static str,
    pub background_tier: BackgroundTier, // 👈 Explicit tier classification
    pub status: TaskStatus,
    pub idle_time_seconds: u64,
    pub cap_token: Option<CapToken>,
    pub has_active_work: bool,
}

impl TaskControlBlock {
    pub fn new(id: u64, name: &'static str, background_tier: BackgroundTier) -> Self {
        Self {
            id,
            name,
            background_tier,
            status: TaskStatus::Ready,
            idle_time_seconds: 0,
            cap_token: None,
            has_active_work: true,
        }
    }

    /// Automatically restores frozen/idle tasks back to normal when user interacts
    pub fn on_user_interaction(&mut self) {
        if self.status == TaskStatus::Frozen || self.status == TaskStatus::Ready {
            self.status = TaskStatus::Ready;
            self.idle_time_seconds = 0; // Reset countdown
        }
    }

    /// Ticks idle timer once per second
    pub fn tick_idle(&mut self) -> Option<&'static str> {
        // 🚨 BACKGROUND TIER CHECK: Privileged background apps bypass idle freeze entirely
        if self.background_tier == BackgroundTier::PrivilegedBackground {
            return None;
        }

        // Standard tier doing active work: keep resetting timer
        if self.has_active_work {
            self.idle_time_seconds = 0;
            return None;
        }

        // Standard tier with zero active work: tick 300s countdown
        if self.status == TaskStatus::Ready || self.status == TaskStatus::Running {
            self.idle_time_seconds += 1;

            if self.idle_time_seconds >= 300 {
                self.status = TaskStatus::Frozen;
                return Some(self.name); // Returns app name to launch toast
            }
        }

        None
    }
}

