// host_kernel/src/sched/domain.rs

use crate::vmm::VCpu;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DomainId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainState {
    Ready,
    Running,
    Paused,
    Halted,
}

pub struct Domain {
    pub id: DomainId,
    pub name: &'static str,
    pub state: DomainState,
    pub vcpu: VCpu,
    pub time_slice_ticks: u32,
    pub remaining_ticks: u32,
}

impl Domain {
    pub fn new(id: DomainId, name: &'static str, vcpu: VCpu, time_slice_ticks: u32) -> Self {
        Self {
            id,
            name,
            state: DomainState::Ready,
            vcpu,
            time_slice_ticks,
            remaining_ticks: time_slice_ticks,
        }
    }

    pub fn reset_quantum(&mut self) {
        self.remaining_ticks = self.time_slice_ticks;
    }
}

