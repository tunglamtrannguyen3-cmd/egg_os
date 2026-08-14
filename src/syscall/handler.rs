use crate::capability::{allocate_with_capability, CapRights};

pub const SYS_ALLOCATE: u64 = 1;
pub const SYS_VERIFY_CAP: u64 = 2;
pub const SYS_FREEZE_TASK: u64 = 3;

pub fn handle_syscall(sys_num: u64, arg1: usize, arg2: usize) -> Result<usize, &'static str> {
    match sys_num {
        SYS_ALLOCATE => {
            let bytes = arg1;
            let owner_tier = arg2 as u8;
            let (ptr, _token) = allocate_with_capability(owner_tier, bytes, CapRights::READ_WRITE)?;
            Ok(ptr)
        }
        SYS_VERIFY_CAP => {
            let token_id = arg1 as u64;
            let req_bytes = arg2;
            crate::capability::GLOBAL_CAP_GATE.lock().verify_access(
                token_id,
                0,
                req_bytes,
                CapRights::READ_ONLY,
            )?;
            Ok(0)
        }
        _ => Err("Invalid System Call ID"),
    }
}
