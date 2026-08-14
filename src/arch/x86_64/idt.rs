// src/arch/x86_64/idt.rs

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    pointer_low: u16,
    gdt_selector: u16,
    options: u16,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub const fn missing() -> Self {
        Self {
            pointer_low: 0,
            gdt_selector: 0,
            options: 0,
            pointer_middle: 0,
            pointer_high: 0,
            reserved: 0,
        }
    }

    pub fn set_handler(&mut self, handler: usize) {
        self.pointer_low = handler as u16;
        self.gdt_selector = 0x08; // Point to Kernel Code Segment in GDT
        self.options = 0x8E00;    // Present, Ring 0, 64-bit Interrupt Gate
        self.pointer_middle = (handler >> 16) as u16;
        self.pointer_high = (handler >> 32) as u32;
        self.reserved = 0;
    }
}

#[repr(C, align(16))]
pub struct Idt {
    entries: [IdtEntry; 256],
}

#[repr(C, packed)]
struct IdtDescriptor {
    limit: u16,
    base: u64,
}

pub static mut IDT: Idt = Idt {
    entries: [IdtEntry::missing(); 256],
};

extern "C" fn generic_exception_handler() {
    crate::arch::log("[x86_64 CPU EXCEPTION TRAPPED — HALTING CORE]\n");
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

pub fn init() {
    unsafe {
        // Register core CPU exception handlers
        IDT.entries[0].set_handler(generic_exception_handler as usize);  // #DE Divide Error
        IDT.entries[8].set_handler(generic_exception_handler as usize);  // #DF Double Fault
        IDT.entries[13].set_handler(generic_exception_handler as usize); // #GP General Protection Fault
        IDT.entries[14].set_handler(generic_exception_handler as usize); // #PF Page Fault

        let descriptor = IdtDescriptor {
            limit: (core::mem::size_of::<Idt>() - 1) as u16,
            base: &IDT as *const _ as u64,
        };

        // Load pointer into CPU's IDTR register
        core::arch::asm!("lidt [{0}]", in(reg) &descriptor, options(readonly, nostack, preserves_flags));
    }
}

