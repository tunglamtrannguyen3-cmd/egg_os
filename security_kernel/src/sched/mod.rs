// security_kernel/src/sched/mod.rs

pub mod task;
pub use task::{BackgroundTier, TaskControlBlock, TaskStatus};
use crate::ui::status_bar::show_job_finished_toast;

pub struct TaskManager {
    tasks: [Option<TaskControlBlock>; 16],
    count: usize,
}

impl TaskManager {
    pub const fn new() -> Self {
        Self {
            tasks: [const { None }; 16],
            count: 0,
        }
    }
}

/// Global scheduler initialization called during kernel boot in main.rs
pub fn init() {
    // Scheduler initialization logic
}

    pub fn spawn(&mut self, name: &'static str, background_tier: BackgroundTier) -> Option<u64> {
        if self.count >= self.tasks.len() {
            return None;
        }
        let id = (self.count + 1) as u64;
        self.tasks[self.count] = Some(TaskControlBlock::new(id, name, background_tier));
        self.count += 1;
        Some(id)
    }

    /// Unfreezes a task when mouse/keyboard interaction is directed at it
    pub fn handle_user_input(&mut self, task_id: u64) {
        if let Some(task) = self.tasks.iter_mut().flatten().find(|t| t.id == task_id) {
            task.on_user_interaction();
        }
    }

    /// Evaluates idle clocks every second
    pub fn on_second_tick(&mut self) {
        for task in self.tasks.iter_mut().flatten() {
            if let Some(app_name) = task.tick_idle() {
                show_job_finished_toast(app_name);
            }
        }
    }


