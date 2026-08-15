pub mod vmx;

extern "C" {
    // Declared in host_kernel/src/vmm/switch.S
    pub fn run_vcpu_loop() -> u64;
}

#[repr(C)]
pub struct VCpu {
    pub id: u32,
    pub registers: GuestRegisters,
}

#[repr(C)]
#[derive(Default)]
pub struct GuestRegisters {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8:  u64,
    pub r9:  u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

impl VCpu {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            registers: GuestRegisters::default(),
        }
    }

    pub fn setup_vmcs(&mut self) {
        // Setup VMCS execution controls, guest state, and host state
        vmx::init_vmcs_region();
    }

    pub fn run(&mut self) {
        loop {
            // Jump into guest VM execution via assembly switcher
            let exit_reason = unsafe { run_vcpu_loop() };

            // Handle VM exit (e.g. Hypercall, EPT violation, Interrupt)
            if !self.handle_exit(exit_reason) {
                break;
            }
        }
    }

    fn handle_exit(&mut self, exit_reason: u64) -> bool {
        match exit_reason {
            // VMCALL (0x12 / 18 on Intel VT-x)
            18 => {
                let call_id = self.registers.rax;
                let arg1 = self.registers.rdi;
                let arg2 = self.registers.rsi;

                let ret = crate::hypercall::handle_hypercall(call_id, arg1, arg2);
                self.registers.rax = ret;
                true // Resume guest execution
            }
            _ => false, // Unknown exit condition; halt VM
        }
    }
}

// Add this near top or bottom of host_kernel/src/vmm/mod.rs
use core::arch::global_asm;

global_asm!(
    ".global run_vcpu_loop",
    "run_vcpu_loop:",
    "  # Assembly VM-entry / VM-exit loop stub",
    "  ret"
);
