use lazy_static::lazy_static;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use super::gdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Bind Double Fault to the isolated IST stack configured in GDT
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault.set_handler_fn(gp_fault_handler);

        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    crate::println!("\n[EXCEPTION] DOUBLE FAULT (error code: {})\n{:#?}", error_code, stack_frame);
    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    let faulting_address = Cr2::read();
    crate::println!(
        "\n[EXCEPTION] PAGE FAULT\nAccessed Address: {:?}\nError Code: {:?}\n{:#?}",
        faulting_address,
        error_code,
        stack_frame
    );
    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn gp_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    crate::println!(
        "\n[EXCEPTION] GENERAL PROTECTION FAULT (error code: {})\n{:#?}",
        error_code,
        stack_frame
    );
    loop {
        x86_64::instructions::hlt();
    }
}