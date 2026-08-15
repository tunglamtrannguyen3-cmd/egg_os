// src/arch/x86_64/gdt.rs

#[repr(C, packed)]
struct GdtDescriptor {
    limit: u16,
    base: u64,
}

#[repr(C, align(8))]
pub struct Gdt {
    entries: [u64; 3],
}

impl Gdt {
    pub const fn new() -> Self {
        Self {
            entries: [
                0x0000000000000000, // Null Segment (Index 0)
                0x00af9a000000ffff, // Kernel Code Segment 64-bit (Selector 0x08, Index 1)
                0x00cf92000000ffff, // Kernel Data Segment 64-bit (Selector 0x10, Index 2)
            ],
        }
    }

    pub fn load(&'static self) {
        let descriptor = GdtDescriptor {
            limit: (core::mem::size_of::<Self>() - 1) as u16,
            base: self as *const _ as u64,
        };

        unsafe {
            core::arch::asm!(
                "lgdt [{0}]",            // Load GDT base pointer into GDTR register
                "push 0x08",             // Push 64-bit Code Segment selector onto stack
                "lea rax, [2f + rip]",   // Load address of label 2 into RAX
                "push rax",              // Push return address onto stack
                "retfq",                 // Far return to reload CS register
                "2:",
                "mov ax, 0x10",          // Load Data Segment selector (0x10)
                "mov ds, ax",            // Reload DS
                "mov es, ax",            // Reload ES
                "mov fs, ax",            // Reload FS
                "mov gs, ax",            // Reload GS
                "mov ss, ax",            // Reload SS
                in(reg) &descriptor,
                out("rax") _,
                options(nostack)
            );
        }
    }
}

pub static GDT: Gdt = Gdt::new();

pub fn init() {
    GDT.load();
}

