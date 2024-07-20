use core::ops::Add;

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{
    VirtioBlkConfig, VirtioBlkDevice, VIRTIO_BLK_F_MQ, VIRTIO_BLK_F_RO, VIRTIO_F_EVENT_IDX,
    VIRTIO_F_INDIRECT_DESC, VIRTIO_F_NOTIFICATION_DATA, VIRTIO_F_RING_PACKED,
};
use crate::driver::virtio::virtq::{Virtq, VIRTQ_AVAIL_F_NO_INTERRUPT, VIRTQ_DESC_LIST_LENGTH};
use crate::driver::virtio::{ACKNOWLEDGE, DRIVER, DRIVER_OK, FEATURE_OK};
use crate::driver::{virtio::VirtioMMIODeivce, Device};
use crate::mm::page::PAGE_SIZE;
use crate::printk;

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

// when mount the device, you should wrap this with the lock.
pub struct VirtioBlkMMIODeivce {
    base: usize,
    size: usize,
    sector_size: usize,
    vq: Box<Virtq>,
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
        unimplemented!("")
    }
}

impl VirtioBlkMMIODeivce {
    fn legacy_queue_init(&mut self) -> Result<(), &'static str> {
        self.write_volatile::<u32>(MMIO_GUEST_PAGE_SIZE_OFFSET, PAGE_SIZE as u32);
        self.write_volatile::<u32>(MMIO_QUEUE_ALIGN_OFFSET, PAGE_SIZE as u32);
        self.write_volatile::<u32>(MMIO_QUEUE_NUM_OFFSET, VIRTQ_DESC_LIST_LENGTH as u32);
        self.vq.set_avail_flags(VIRTQ_AVAIL_F_NO_INTERRUPT as u16);
        let pfn = self.vq.self_addr() >> 12;
        self.write_volatile::<u32>(MMIO_QUEUE_PFN_OFFSET, pfn as u32);
        Ok(())
    }
    fn queue_init(&self) -> Result<(), &'static str> {
        let desc_addr = self.vq.desc_addr();
        let avail_addr = self.vq.avail_addr();
        let used_addr = self.vq.used_addr();
        self.write_volatile::<u32>(MMIO_QUEUE_DESC_LOW_OFFSET, desc_addr as u32);
        self.write_volatile::<u32>(MMIO_QUEUE_DESC_HIGH_OFFSET, (desc_addr >> 32) as u32);
        self.write_volatile::<u32>(MMIO_QUEUE_DRIVER_LOW_OFFSET, avail_addr as u32);
        self.write_volatile::<u32>(MMIO_QUEUE_DRIVER_HIGH_OFFSET, (avail_addr >> 32) as u32);
        self.write_volatile::<u32>(MMIO_QUEUE_DEVICE_LOW_OFFSET, used_addr as u32);
        self.write_volatile::<u32>(MMIO_QUEUE_DEVICE_HIGH_OFFSET, (used_addr >> 32) as u32);
        Ok(())
    }
    fn check_magic(&self) -> Result<(), &'static str> {
        let magic = self.read_volatile::<u32>(MMIO_MAGIC_OFFSET);
        if magic != VIRTIO_MMIO_MAGIC {
            return Err("the magic number is not correct.");
        }
        Ok(())
    }
}

impl VirtioMMIODeivce for VirtioBlkMMIODeivce {
    fn mmio_init(base: usize, size: usize) -> Result<Box<Self>, &'static str> {
        let mut device = Box::new(VirtioBlkMMIODeivce {
            base,
            size,
            sector_size: 512,
            vq: Box::new(Virtq::init()),
            cfg: None,
        });
        let mut features: u32 = 0;
        let mut status: u32 = 0;
        device.check_magic()?;
        let version = device.read_volatile::<u32>(MMIO_VERSION_OFFSET);
        device.write_volatile::<u32>(MMIO_STATUS_OFFSET, status);
        status |= ACKNOWLEDGE;
        device.write_volatile::<u32>(MMIO_STATUS_OFFSET, status);
        status |= DRIVER;
        device.write_volatile::<u32>(MMIO_STATUS_OFFSET, status);
        device.write_volatile::<u32>(MMIO_DEVICE_FEATURES_SEL_OFFSET, 0);
        features = device.read_volatile::<u32>(MMIO_DEVICE_FEATURES_OFFSET);
        features &= !(1 << VIRTIO_BLK_F_RO);
        features &= !(1 << VIRTIO_BLK_F_MQ);
        features &= !(1 << VIRTIO_F_INDIRECT_DESC);
        features &= !(1 << VIRTIO_F_EVENT_IDX);

        device.write_volatile::<u32>(MMIO_DRIVER_FEATURES_OFFSET, features);
        device.write_volatile::<u32>(MMIO_DEVICE_FEATURES_SEL_OFFSET, 1);
        features = device.read_volatile::<u32>(MMIO_DEVICE_FEATURES_OFFSET);
        features &= !(1 << VIRTIO_F_NOTIFICATION_DATA);
        features &= !(1 << VIRTIO_F_RING_PACKED);
        device.write_volatile::<u32>(MMIO_DRIVER_FEATURES_SEL_OFFSET, 1);
        device.write_volatile::<u32>(MMIO_DRIVER_FEATURES_OFFSET, features);
        status |= FEATURE_OK;
        device.write_volatile::<u32>(MMIO_STATUS_OFFSET, status);

        if device.read_volatile::<u32>(MMIO_STATUS_OFFSET) & FEATURE_OK == 0 {
            return Err("the device does not support the features.");
        }
        device.write_volatile::<u32>(MMIO_QUEUE_SEL_OFFSET, 0);
        if device.read_volatile::<u32>(MMIO_QUEUE_READY_OFFSET) != 0 {
            return Err("the qeuue 0 is not ready.");
        }
        if (device.read_volatile::<u32>(MMIO_QUEUE_NUM_MAX_OFFSET) as usize)
            < VIRTQ_DESC_LIST_LENGTH
        {
            return Err("the queue size is too LARGE.");
        }
        if version == VIRTIO_BLK_LEGACY_VERSION {
            device.legacy_queue_init()?;
        } else if version == 1 {
            device.queue_init()?;
        } else {
            return Err("the version is not supported.");
        }
        status |= DRIVER_OK;
        device.write_volatile::<u32>(MMIO_STATUS_OFFSET, status);
        Ok(device)
    }
}

const VIRTIO_BLK_LEGACY_VERSION: u32 = 0;
