// host_kernel/src/sched/mod.rs

pub mod domain;

pub use domain::{Domain, DomainId, DomainState};
use crate::vmm::VCpu;

pub const MAX_GUEST_DOMAINS: usize = 4;

pub struct HostScheduler {
    domains: [Option<Domain>; MAX_GUEST_DOMAINS],
    current_index: usize,
    count: usize,
}

impl HostScheduler {
    pub const fn new() -> Self {
        Self {
            domains: [None, None, None, None],
            current_index: 0,
            count: 0,
        }
    }

    /// Registers a new hardware-isolated guest domain (e.g. security_kernel)
    pub fn register_domain(
        &mut self,
        name: &'static str,
        vcpu: VCpu,
        time_slice_ticks: u32,
    ) -> Option<DomainId> {
        if self.count >= MAX_GUEST_DOMAINS {
            return None;
        }

        let domain_id = DomainId(self.count as u32);
        let domain = Domain::new(domain_id, name, vcpu, time_slice_ticks);

        self.domains[self.count] = Some(domain);
        self.count += 1;
        Some(domain_id)
    }

    /// Called on every LAPIC timer interrupt or VMX preemption exit
    pub fn on_timer_tick(&mut self) {
        if self.count == 0 {
            return;
        }

        if let Some(current_domain) = &mut self.domains[self.current_index] {
            if current_domain.remaining_ticks > 0 {
                current_domain.remaining_ticks -= 1;
            }

            // Time slice expired: trigger domain context switch
            if current_domain.remaining_ticks == 0 {
                current_domain.reset_quantum();
                self.switch_to_next_domain();
            }
        }
    }

    /// Preemptively switches control to the next active Guest VM domain
    pub fn switch_to_next_domain(&mut self) {
        if self.count <= 1 {
            return; // Only 1 domain registered; no need to switch
        }

        // Mark current domain as Ready
        if let Some(domain) = &mut self.domains[self.current_index] {
            if domain.state == DomainState::Running {
                domain.state = DomainState::Ready;
            }
        }

        // Find next runnable domain in round-robin sequence
        let mut next = (self.current_index + 1) % self.count;
        for _ in 0..self.count {
            if let Some(domain) = &self.domains[next] {
                if domain.state == DomainState::Ready {
                    self.current_index = next;
                    break;
                }
            }
            next = (next + 1) % self.count;
        }

        // Launch selected domain vCPU
        if let Some(domain) = &mut self.domains[self.current_index] {
            domain.state = DomainState::Running;
            domain.vcpu.run(); // Calls VMLAUNCH / VMRESUME in host_kernel/src/vmm/switch.S
        }
    }

    /// Pauses a domain (e.g. if security_kernel reported all tasks are Frozen/Idle)
    pub fn pause_domain(&mut self, id: DomainId) {
        if let Some(domain) = self.domains.iter_mut().flatten().find(|d| d.id == id) {
            domain.state = DomainState::Paused;
            if self.current_index == id.0 as usize {
                self.switch_to_next_domain();
            }
        }
    }

    /// Resumes a paused domain when hardware interrupt arrives
    pub fn unpause_domain(&mut self, id: DomainId) {
        if let Some(domain) = self.domains.iter_mut().flatten().find(|d| d.id == id) {
            domain.state = DomainState::Ready;
        }
    }
}

