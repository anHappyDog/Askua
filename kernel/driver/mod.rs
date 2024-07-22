use core::ops::Add;

use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, vec::Vec};
use virtio::{blk::VirtioBlkDevice, mmio::VirtioMMIODeivce, net::VirtioNetDevice};

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

enum NetDevice {
    VirtioNet(Arc<IrqSafeSpinlock<Box<dyn VirtioNetDevice>>>),
}

static BLOCK_DEVICES: IrqSafeSpinlock<BTreeMap<usize, BlockDevice>> =
    IrqSafeSpinlock::new(BTreeMap::new());
static NET_DEVICES: IrqSafeSpinlock<BTreeMap<usize, BlockDevice>> =
    IrqSafeSpinlock::new(BTreeMap::new());

pub trait Device {
    fn read_volatile<T>(&self, offset: usize) -> T
    where
        T: Add;
    fn write_volatile<T>(&self, offset: usize, value: T);
}

pub fn dev_init() {
    
}
