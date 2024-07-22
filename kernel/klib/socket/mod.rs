use core::sync::atomic::{AtomicBool, AtomicI8};

mod consts;

pub struct SocketAddr {
    family: u16,
    port: u16,
    addr: u32,
    zero: [u8; 8],
}

struct SokcetState {
    state_lock: AtomicBool,
    is_close: bool,
    opposite_write_close: bool,
}
