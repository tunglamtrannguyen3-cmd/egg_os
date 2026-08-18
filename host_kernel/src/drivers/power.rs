use x86_64::instructions::port::Port;

#[repr(C, packed)]
struct AcpiHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

#[repr(C, packed)]
struct Fadt {
    header: AcpiHeader,
    firmware_ctrl: u32,
    dsdt: u32,
    reserved: u8,
    preferred_pm_profile: u8,
    sci_interrupt: u16,
    smi_cmd_port: u32,
    acpi_enable: u8,
    acpi_disable: u8,
    s4bios_req: u8,
    pstate_cnt: u8,
    pm1a_event_blk: u32,
    pm1b_event_blk: u32,
    pm1a_cnt_blk: u32,
    pm1b_cnt_blk: u32,
}

pub unsafe fn enable_power_button(pm1a_evt_blk: u16) {
    let mut port = Port::<u16>::new(pm1a_evt_blk + 2);
    let current_val = port.read();
    port.write(current_val | (1 << 8));
}

pub unsafe fn shutdown(rsdp_addr: u64) -> ! {
    if rsdp_addr != 0 {
        let rsdp_ptr = rsdp_addr as *const u8;
        let rsdt_addr = *(rsdp_ptr.add(16) as *const u32);
        let rsdt_header = rsdt_addr as *const AcpiHeader;
        let entry_count = ((*rsdt_header).length - 36) / 4;
        let entries = (rsdt_addr as *const u8).add(36) as *const u32;

        let mut pm1a_cnt = 0u32;

        for i in 0..entry_count {
            let table_ptr = *entries.add(i as usize) as *const AcpiHeader;
            if &(*table_ptr).signature == b"FACP" {
                let fadt = table_ptr as *const Fadt;
                pm1a_cnt = (*fadt).pm1a_cnt_blk;
                break;
            }
        }

        if pm1a_cnt != 0 {
            let slp_s5 = (5 << 10) | (1 << 13);
            let mut port = Port::<u16>::new(pm1a_cnt as u16);
            port.write(slp_s5);
        }
    }

    loop {
        x86_64::instructions::hlt();
    }
}
