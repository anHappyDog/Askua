use alloc::boxed::Box;

pub mod blk;
pub mod gpu;
pub mod net;
pub mod virtq;
pub trait VirtioDevice {}
pub trait VirtioMMIODeivce {
    fn mmio_init(base: usize, size: usize) -> Result<Box<Self>, &'static str>;
}
pub trait VirtioPCIDevice {
    fn pci_init() -> Self;
}
pub trait VirtioChannelIODevice {
    fn channel_init() -> Self;
}

const ACKNOWLEDGE: u32 = 1;
const DRIVER: u32 = 2;
const DRIVER_OK: u32 = 4;
const FEATURE_OK: u32 = 8;
const FAILED: u32 = 0x80;
const DEVICE_NEEDS_RESET: u32 = 0x40;
