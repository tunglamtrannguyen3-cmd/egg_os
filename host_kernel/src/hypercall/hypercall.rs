pub const HYPERCALL_IPC_SEND: u64 = 0x01;
pub const HYPERCALL_IPC_RECV: u64 = 0x02;
pub const HYPERCALL_ALLOC_FRAME: u64 = 0x03;

pub fn handle_hypercall(call_id: u64, _arg1: u64, _arg2: u64) -> u64 {
    match call_id {
        HYPERCALL_IPC_SEND => {
            // Transfer shared memory ring
            0 // Success
        }

        HYPERCALL_ALLOC_FRAME => {
            // Safe guest frame allocation
            let mut mem_mgr = crate::memory::ept::HOST_PHYSICAL_ALLOCATOR.lock();

            mem_mgr.allocate_frame()
        }

        HYPERCALL_IPC_RECV => {
            // IPC handler stub for v0.4.0
            0 // Success
        }

        _ => {
            u64::MAX // Unknown hypercall
        }
    }
}
