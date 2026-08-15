#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessMemState {
    Active,
    FrozenLocked,
}

pub struct MemoryRegionState {
    pub task_id: u64,
    pub base_addr: usize,
    pub size_bytes: usize,
    pub state: ProcessMemState,
}

impl MemoryRegionState {
    pub fn new(task_id: u64, base_addr: usize, size_bytes: usize) -> Self {
        Self {
            task_id,
            base_addr,
            size_bytes,
            state: ProcessMemState::Active,
        }
    }

    pub fn lock(&mut self) {
        self.state = ProcessMemState::FrozenLocked;
        crate::arch::log("[EggOS Memory]: Region locked for frozen background process.\n");
    }

    pub fn unlock(&mut self) {
        self.state = ProcessMemState::Active;
        crate::arch::log("[EggOS Memory]: Region unlocked for active execution.\n");
    }
}
