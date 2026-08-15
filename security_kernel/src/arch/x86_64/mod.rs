// src/arch/x86_64/mod.rs

pub mod gdt;
pub mod idt;
pub mod serial;

pub fn init() {
    serial::init();
    gdt::init(); // Load GDT into GDTR
    idt::init(); // Load IDT into IDTR
    serial::print_str("[EggOS Arch x86_64: Serial, GDT, & IDT Initialized]\n");
}

