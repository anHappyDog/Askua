use core::ops::Add;

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use virtio::{blk::VirtioBlkDevice, mmio::VirtioMMIODeivce};

use crate::lock::irq_safe::spin::IrqSafeSpinlock;

pub mod clint;
pub mod fdt;
pub mod nic;
pub mod plic;
pub mod rtc;
pub mod sdcard;
pub mod uart;
pub mod virtio;

enum BlockDevice {
    VirtioBlk(Arc<IrqSafeSpinlock<Box<dyn VirtioBlkDevice>>>),
}

static BLOCK_DEVICES: IrqSafeSpinlock<Vec<BlockDevice>> = IrqSafeSpinlock::new(Vec::new());

pub trait Device {
    fn read_volatile<T>(&self, offset: usize) -> T
    where
        T: Add;
    fn write_volatile<T>(&self, offset: usize, value: T);
}

pub fn dev_init() {
    BLOCK_DEVICES
        .lock()
        .push(BlockDevice::VirtioBlk(Arc::new(IrqSafeSpinlock::new(
            virtio::blk::mmio::VirtioBlkMMIODeivce::mmio_init(0x10001000, 0x1000).expect("sa"),
        ))));
    
}
