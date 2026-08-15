// src/capability/gate.rs

use super::token::{CapRights, CapToken, ResourceKind};
use spin::Mutex;

pub const MAX_CAPABILITIES: usize = 32;

pub struct CapGate {
    slots: [Option<CapToken>; MAX_CAPABILITIES],
    next_id: u64,
}

impl CapGate {
    pub const fn new() -> Self {
        Self {
            slots: [None; MAX_CAPABILITIES],
            next_id: 1000,
        }
    }

    /// Mint a brand new capability token for a system resource
    pub fn mint(
        &mut self,
        owner_tier: u8,
        kind: ResourceKind,
        addr: usize,
        size: usize,
        rights: CapRights,
    ) -> Result<CapToken, &'static str> {
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                let token = CapToken::new(self.next_id, owner_tier, kind, addr, size, rights);
                self.next_id += 1;
                *slot = Some(token);
                return Ok(token);
            }
        }
        Err("Capability Gate Error: Token Table Full")
    }

    /// Verify whether a token exists, is active, covers the address range, and satisfies rights
    pub fn verify_access(
        &self,
        token_id: u64,
        target_addr: usize,
        requested_bytes: usize,
        required_rights: CapRights,
    ) -> Result<(), &'static str> {
        for slot in self.slots.iter().flatten() {
            if slot.id == token_id {
                if !slot.active {
                    return Err("Access Denied: Capability Token Revoked");
                }

                // Bounds verification
                if target_addr < slot.resource_addr
                    || (target_addr + requested_bytes) > (slot.resource_addr + slot.size_bytes)
                {
                    return Err("Access Denied: Requested Address Outside Capability Bounds");
                }

                // Rights verification
                if !slot.rights.satisfies(required_rights) {
                    return Err("Access Denied: Insufficient Capability Rights");
                }

                return Ok(());
            }
        }
        Err("Access Denied: Invalid Capability Token ID")
    }

    /// Revoke a capability token instantly across all rings
    pub fn revoke(&mut self, token_id: u64) -> bool {
        for slot in self.slots.iter_mut() {
            if let Some(ref mut token) = slot {
                if token.id == token_id {
                    token.active = false;
                    return true;
                }
            }
        }
        false
    }
}

pub static GLOBAL_CAP_GATE: Mutex<CapGate> = Mutex::new(CapGate::new());

