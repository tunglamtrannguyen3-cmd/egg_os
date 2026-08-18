use crate::vmm::VCpu;

pub fn handle_vmexit(_vcpu: &mut VCpu, _exit_reason: u64) {
    // Handle VMCALLs, EPT violations, or interrupts
}
