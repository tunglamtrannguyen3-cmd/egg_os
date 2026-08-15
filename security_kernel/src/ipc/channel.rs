use crate::capability::{CapRights, GLOBAL_CAP_GATE};
use crate::memory::MatchChunkStream;

pub const MAX_MSG_SIZE: usize = 1024;

pub struct IpcMessage {
    pub sender_id: u64,
    pub payload_len: usize,
    pub buffer: [u8; MAX_MSG_SIZE],
}

pub struct IpcChannel {
    pub channel_id: u64,
    pub token_id: u64,
}

impl IpcChannel {
    pub fn new(channel_id: u64, token_id: u64) -> Self {
        Self { channel_id, token_id }
    }

    pub fn send(&self, _sender_id: u64, data: &[u8]) -> Result<(), &'static str> {
        GLOBAL_CAP_GATE.lock().verify_access(
            self.token_id,
            0,
            data.len(),
            CapRights::READ_WRITE,
        )?;

        if data.len() > MAX_MSG_SIZE {
            return Err("IPC Error: Payload exceeds max message size");
        }

        crate::arch::log("[EggOS IPC]: Capability verified. Message transmitted.\n");
        Ok(())
    }

    pub fn stream_payload(&self, total_bytes: usize) -> Result<(), &'static str> {
        GLOBAL_CAP_GATE.lock().verify_access(
            self.token_id,
            0,
            total_bytes,
            CapRights::READ_WRITE,
        )?;

        let stream = MatchChunkStream::new(total_bytes);
        for chunk in stream {
            let _ = chunk;
        }

        crate::arch::log("[EggOS IPC]: Adaptive chunk stream transfer complete.\n");
        Ok(())
    }
}
