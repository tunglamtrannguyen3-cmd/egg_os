pub fn handle_hypercall(call_id: u64, _arg1: u64, _arg2: u64) -> u64 {
    match call_id {
        _ => 0,
    }
}
