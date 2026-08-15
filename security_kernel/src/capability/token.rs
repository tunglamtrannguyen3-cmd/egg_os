// src/capability/token.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapRights {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub grant: bool,
}

impl CapRights {
    pub const NONE: Self = Self { read: false, write: false, execute: false, grant: false };
    pub const READ_ONLY: Self = Self { read: true, write: false, execute: false, grant: false };
    pub const READ_WRITE: Self = Self { read: true, write: true, execute: false, grant: false };
    pub const EXECUTE: Self = Self { read: false, write: false, execute: true, grant: false };
    pub const FULL_CONTROL: Self = Self { read: true, write: true, execute: true, grant: true };

    /// Capability Attenuation: Derives child rights guaranteed not to exceed parent rights
    pub fn attenuate(&self, requested: Self) -> Self {
        Self {
            read: self.read && requested.read,
            write: self.write && requested.write,
            execute: self.execute && requested.execute,
            grant: self.grant && requested.grant,
        }
    }

    /// Check if current rights satisfy a required set of permission flags
    pub fn satisfies(&self, required: Self) -> bool {
        (!required.read || self.read) &&
        (!required.write || self.write) &&
        (!required.execute || self.execute) &&
        (!required.grant || self.grant)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    MemoryRegion,
    MmioDevice,
    IpcChannel,
    DriverControl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapToken {
    pub id: u64,
    pub owner_tier: u8, // Ring tier (Ring 1 System, Ring 2 User)
    pub resource_kind: ResourceKind,
    pub resource_addr: usize,
    pub size_bytes: usize,
    pub rights: CapRights,
    pub active: bool,
}

impl CapToken {
    pub fn new(
        id: u64,
        owner_tier: u8,
        resource_kind: ResourceKind,
        resource_addr: usize,
        size_bytes: usize,
        rights: CapRights,
    ) -> Self {
        Self {
            id,
            owner_tier,
            resource_kind,
            resource_addr,
            size_bytes,
            rights,
            active: true,
        }
    }

    /// Derive a child token with equal or reduced rights
    pub fn derive(&self, child_id: u64, requested_rights: CapRights) -> Option<Self> {
        if !self.active || !self.rights.grant {
            return None;
        }

        Some(Self {
            id: child_id,
            owner_tier: self.owner_tier,
            resource_kind: self.resource_kind,
            resource_addr: self.resource_addr,
            size_bytes: self.size_bytes,
            rights: self.rights.attenuate(requested_rights),
            active: true,
        })
    }
}

