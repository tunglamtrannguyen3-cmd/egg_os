pub mod gate;
pub mod token;

pub use gate::GLOBAL_CAP_GATE;
pub use token::{CapRights, CapToken, ResourceKind};

pub fn init() {
    let mut gate = GLOBAL_CAP_GATE.lock();

    let _ = gate.mint(
        2,
        ResourceKind::MmioDevice,
        0xFD000000,
        1920 * 1080 * 4,
        CapRights::READ_WRITE,
    );

    let _ = gate.mint(
        1,
        ResourceKind::MmioDevice,
        0x10001000,
        0x1000,
        CapRights::FULL_CONTROL,
    );

    crate::arch::log("[EggOS Capability Engine: Tokens, Gates & Revocation Online]\n");
}

pub fn allocate_with_capability(
    owner_tier: u8,
    bytes: usize,
    rights: CapRights,
) -> Result<(usize, CapToken), &'static str> {
    let ptr = crate::memory::allocate(bytes).ok_or("Memory Allocation Failed: Out of RAM")?;

    let token = GLOBAL_CAP_GATE.lock().mint(
        owner_tier,
        ResourceKind::MemoryRegion,
        ptr,
        bytes,
        rights,
    )?;

    Ok((ptr, token))
}
