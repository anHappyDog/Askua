pub mod blk;
pub mod gpu;
pub mod net;

pub trait VirtioDevice {}

const ACKNOWLEDGE: u32 = 1;
const DRIVER: u32 = 2;
const DRIVER_OK: u32 = 4;
const FEATURE_OK: u32 = 8;
const FAILED: u32 = 0x80;
const DEVICE_NEEDS_RESET: u32 = 0x40;
