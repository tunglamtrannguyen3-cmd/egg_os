pub mod vmexit;
pub mod vmx;

use core::arch::global_asm;

// Context switch stub between Hypervisor (Host) and Security Kernel (Guest)
global_asm!(
    ".global run_vcpu_loop",
    "run_vcpu_loop:",
    "  # 1. Preserve host registers on host stack",
    "  push rbp",
    "  push rbx",
    "  push r12",
    "  push r13",
    "  push r14",
    "  push r15",
    
    "  # 2. Resume guest execution (fall back to launch on initial run)",
    "  vmresume",
    "  vmlaunch",
    
    "  # 3. Restore host registers upon VM-Exit",
    "  pop r15",
    "  pop r14",
    "  pop r13",
    "  pop r12",
    "  pop rbx",
    "  pop rbp",
    "  ret"
);

extern "C" {
    pub fn run_vcpu_loop() -> u64;
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
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

#[repr(C)]
pub struct VCpu {
    pub id: u32,
    pub registers: GuestRegisters,
}

impl VCpu {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            registers: GuestRegisters::default(),
        }
    }

    pub fn setup_vmcs(&mut self, guest_rip: u64) {
        vmx::init_vmcs_region();

        // Write guest entry point to VMCS field 0x0000681E (GUEST_RIP)
        unsafe {
            let _ = x86::bits64::vmx::vmwrite(0x0000681E, guest_rip);
        }
    }

    pub fn run(&mut self) -> u64 {
        unsafe { run_vcpu_loop() }
    }

    pub fn get_reg(&self, index: u8) -> u64 {
        match index {
            0 => self.registers.rax,
            1 => self.registers.rbx,
            2 => self.registers.rcx,
            3 => self.registers.rdx,
            _ => 0,
        }
    }

    pub fn set_reg(&mut self, index: u8, val: u64) {
        match index {
            0 => self.registers.rax = val,
            1 => self.registers.rbx = val,
            2 => self.registers.rcx = val,
            3 => self.registers.rdx = val,
            _ => {}
        }
    }

    pub fn advance_guest_rip(&mut self, bytes: u64) {
        let current_rip = unsafe { x86::bits64::vmx::vmread(0x0000681E).unwrap_or(0) };
        unsafe {
            let _ = x86::bits64::vmx::vmwrite(0x0000681E, current_rip + bytes);
        }
    }
}

