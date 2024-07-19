use core::ops::Add;

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{VirtioBlkConfig, VirtioBlkDevice};
use crate::driver::virtio::virtq::Virtq;
use crate::driver::{virtio::VirtioMMIODeivce, Device};

const VIRTIO_MMIO_MAGIC: u32 = 0x74726976;

const MMIO_MAGIC_OFFSET: usize = 0x0;
const MMIO_VERSION_OFFSET: usize = 0x4;
const MMIO_DEVICE_ID_OFFSET: usize = 0x8;
const MMIO_VENDOR_ID_OFFSET: usize = 0xc;
const MMIO_DEVICE_FEATURES_OFFSET: usize = 0x10;
const MMIO_DEVICE_FEATURES_SEL_OFFSET: usize = 0x14;
const MMIO_DRIVER_FEATURES_OFFSET: usize = 0x20;
const MMIO_DRIVER_FEATURES_SEL_OFFSET: usize = 0x24;
const MMIO_GUEST_PAGE_SIZE_OFFSET: usize = 0x28;
const MMIO_QUEUE_SEL_OFFSET: usize = 0x30;
const MMIO_QUEUE_NUM_MAX_OFFSET: usize = 0x34;
const MMIO_QUEUE_NUM_OFFSET: usize = 0x38;
const MMIO_QUEUE_ALIGN_OFFSET: usize = 0x3c;
const MMIO_QUEUE_PFN_OFFSET: usize = 0x40;
const MMIO_QUEUE_READY_OFFSET: usize = 0x44;
const MMIO_QUEUE_NOTIFY_OFFSET: usize = 0x50;
const MMIO_INTERRUPT_STATUS_OFFSET: usize = 0x60;
const MMIO_INTERRUPT_ACK_OFFSET: usize = 0x64;
const MMIO_STATUS_OFFSET: usize = 0x70;
const MMIO_QUEUE_DESC_LOW_OFFSET: usize = 0x80;
const MMIO_QUEUE_DESC_HIGH_OFFSET: usize = 0x84;
const MMIO_QUEUE_DRIVER_LOW_OFFSET: usize = 0x90;
const MMIO_QUEUE_DRIVER_HIGH_OFFSET: usize = 0x94;
const MMIO_QUEUE_DEVICE_LOW_OFFSET: usize = 0xa0;
const MMIO_QUEUE_DEVICE_HIGH_OFFSET: usize = 0xa4;
const MMIO_SHMSEL_OFFSET: usize = 0xac;
const MMIO_SHMLEN_LOW_OFFSET: usize = 0xb0;
const MMIO_SHMLEN_HIGH_OFFSET: usize = 0xb4;
const MMIO_SHMBASE_LOW_OFFSET: usize = 0xb8;
const MMIO_SHMBASE_HIGH_OFFSET: usize = 0xbc;
const MMIO_QUEUE_RST_OFFSET: usize = 0xc0;
const MMIO_CONFIG_GEN_OFFSET: usize = 0xfc;
const MMIO_CONFIG_OFFSET: usize = 0x100;

struct VirtioBlkMMIODeivce {
    base: usize,
    size: usize,
    sector_size: usize,
    vqs: Virtq,
    cfg: Option<VirtioBlkConfig>,
}

impl Device for VirtioBlkMMIODeivce {
    fn read_volatile<T>(&self, offset: usize) -> T
    where
        T: Add,
    {
        unsafe { ((self.base + offset) as *const T).read_volatile() }
    }

    fn write_volatile<T>(&self, offset: usize, value: T) {
        unsafe {
            ((self.base + offset) as *mut T).write_volatile(value);
        }
    }
}

impl VirtioBlkDevice for VirtioBlkMMIODeivce {
    fn read_sectors(
        &mut self,
        data: &mut [u8],
        sector: u64,
        count: u32,
    ) -> Result<(), &'static str> {
        if data.len() < self.sector_size * count as usize {
            return Err("the data buffer is too short for conatining the data.");
        }
        
        Ok(())
    }

    fn write_sectors(&mut self, data: &[u8], sector: u64, count: u32) -> Result<(), &'static str> {
        unimplemented!()
    }
}

impl VirtioBlkMMIODeivce {
    fn legacy_mmio_init(base: usize, size: usize) -> Result<Box<Self>, &'static str> {
        unimplemented!("legacy_mmio_init")
    }
}

impl VirtioMMIODeivce for VirtioBlkMMIODeivce {
    fn mmio_init(base: usize, size: usize) -> Result<Box<Self>, &'static str> {
        unimplemented!("mmio_init")
    }
}
