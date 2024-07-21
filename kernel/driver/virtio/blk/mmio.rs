use core::ops::Add;

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{
    VirtioBlkConfig, VirtioBlkDevice, VirtqBlkReq, DEVICE_ID_BLOCK, VIRTIO_BLK_F_MQ,
    VIRTIO_BLK_F_RO, VIRTIO_BLK_T_IN, VIRTIO_F_EVENT_IDX, VIRTIO_F_INDIRECT_DESC,
    VIRTIO_F_NOTIFICATION_DATA, VIRTIO_F_RING_PACKED,
};
use crate::driver::virtio::blk::{VIRTIO_BLK_S_OK, VIRTIO_BLK_T_OUT};
use crate::driver::virtio::mmio::consts::{
    MMIO_DEVICE_FEATURES_OFFSET, MMIO_DEVICE_FEATURES_SEL_OFFSET, MMIO_DEVICE_ID_OFFSET,
    MMIO_DRIVER_FEATURES_OFFSET, MMIO_DRIVER_FEATURES_SEL_OFFSET, MMIO_GUEST_PAGE_SIZE_OFFSET,
    MMIO_MAGIC_OFFSET, MMIO_QUEUE_ALIGN_OFFSET, MMIO_QUEUE_DESC_HIGH_OFFSET,
    MMIO_QUEUE_DESC_LOW_OFFSET, MMIO_QUEUE_DEVICE_HIGH_OFFSET, MMIO_QUEUE_DEVICE_LOW_OFFSET,
    MMIO_QUEUE_DRIVER_HIGH_OFFSET, MMIO_QUEUE_DRIVER_LOW_OFFSET, MMIO_QUEUE_NOTIFY_OFFSET,
    MMIO_QUEUE_NUM_MAX_OFFSET, MMIO_QUEUE_NUM_OFFSET, MMIO_QUEUE_PFN_OFFSET,
    MMIO_QUEUE_READY_OFFSET, MMIO_QUEUE_SEL_OFFSET, MMIO_STATUS_OFFSET, MMIO_VERSION_OFFSET,
    VIRTIO_BLK_LEGACY_VERSION, VIRTIO_BLK_MODERN_VERSION, VIRTIO_MMIO_MAGIC,
};
use crate::driver::virtio::mmio::VirtioMMIODeivce;
use crate::driver::virtio::virtq::{
    Virtq, VIRTQ_AVAIL_F_NO_INTERRUPT, VIRTQ_DESC_F_NEXT, VIRTQ_DESC_F_WRITE,
    VIRTQ_DESC_LIST_LENGTH,
};
use crate::driver::virtio::{ACKNOWLEDGE, DRIVER, DRIVER_OK, FEATURE_OK};
use crate::driver::Device;
use crate::mm::page::PAGE_SIZE;
use crate::printk;

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

const MAX_USED_DESC_ONCE_CNT: usize = 32;

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
        let mut alloced_desc_list: [u16; MAX_USED_DESC_ONCE_CNT] = [0; MAX_USED_DESC_ONCE_CNT];
        let mut used_desc = 0;
        let mut req = VirtqBlkReq {
            sector,
            type_: VIRTIO_BLK_T_IN as u32,
            data,
            reserved: 0,
            status: 0,
        };
        let need_desc: usize = count as usize + 2;
        if need_desc > MAX_USED_DESC_ONCE_CNT {
            return Err("the data request is too large to finish in one time.");
        }
        for i in 0..need_desc {
            alloced_desc_list[used_desc] = self.vq.desc_alloc().ok_or("no free desc currently.")?;
            used_desc += 1;
        }
        let idx = self.vq.get_desc_idx();
        let desc = self.vq.get_desc_mut(alloced_desc_list[0]);
        desc.set_addr(&req as *const _ as u64);
        desc.set_len(16);
        desc.set_flags(VIRTQ_DESC_F_NEXT as u16);
        desc.set_next(alloced_desc_list[1] as u16);
        for i in 1..need_desc - 1 {
            let desc = self.vq.get_desc_mut(alloced_desc_list[i]);
            desc.set_addr(
                data.as_ptr()
                    .wrapping_add((i - 1) * self.sector_size as usize) as u64,
            );
            desc.set_flags((VIRTQ_DESC_F_NEXT | VIRTQ_DESC_F_WRITE) as u16);
            desc.set_len(self.sector_size as u32);
            desc.set_next(alloced_desc_list[i as usize + 1] as u16);
        }
        let mut desc = self.vq.get_desc_mut(alloced_desc_list[need_desc - 1]);
        desc.set_addr(&req.status as *const _ as u64);
        desc.set_flags(VIRTQ_DESC_F_WRITE);
        desc.set_len(1);
        desc.set_next(0);
        self.vq.fill_avail(alloced_desc_list[0]);
        self.write_volatile::<u32>(MMIO_QUEUE_NOTIFY_OFFSET, 0);
        while idx + 1 != self.vq.get_used_idx() {}
        assert!(req.status == VIRTIO_BLK_S_OK);
        //free the desc
        Ok(())
    }

    fn write_sectors(&mut self, data: &[u8], sector: u64, count: u32) -> Result<(), &'static str> {
        if data.len() < self.sector_size * count as usize {
            return Err("the data buffer is too short for conatining the data.");
        }
        let mut alloced_desc_list: [u16; MAX_USED_DESC_ONCE_CNT] = [0; MAX_USED_DESC_ONCE_CNT];
        let mut used_desc = 0;
        let mut req = VirtqBlkReq {
            sector,
            type_: VIRTIO_BLK_T_OUT,
            data,
            reserved: 0,
            status: 0,
        };
        let need_desc: usize = count as usize + 2;
        if need_desc > MAX_USED_DESC_ONCE_CNT {
            return Err("the data request is too large to finish in one time.");
        }
        for i in 0..need_desc {
            alloced_desc_list[used_desc] = self.vq.desc_alloc().ok_or("no free desc currently.")?;
            used_desc += 1;
        }
        let idx = self.vq.get_desc_idx();
        let desc = self.vq.get_desc_mut(alloced_desc_list[0]);
        desc.set_addr(&req as *const _ as u64);
        desc.set_len(16);
        desc.set_flags(VIRTQ_DESC_F_NEXT as u16);
        desc.set_next(alloced_desc_list[1] as u16);
        for i in 1..need_desc - 1 {
            let desc = self.vq.get_desc_mut(alloced_desc_list[i]);
            desc.set_addr(
                data.as_ptr()
                    .wrapping_add((i - 1) * self.sector_size as usize) as u64,
            );
            desc.set_flags((VIRTQ_DESC_F_NEXT) as u16);
            desc.set_len(self.sector_size as u32);
            desc.set_next(alloced_desc_list[i as usize + 1] as u16);
        }
        let mut desc = self.vq.get_desc_mut(alloced_desc_list[need_desc - 1]);
        desc.set_addr(&req.status as *const _ as u64);
        desc.set_flags(VIRTQ_DESC_F_WRITE);
        desc.set_len(1);
        desc.set_next(0);
        self.vq.fill_avail(alloced_desc_list[0]);
        self.write_volatile::<u32>(MMIO_QUEUE_NOTIFY_OFFSET, 0);
        while idx + 1 != self.vq.get_used_idx() {}
        assert!(req.status == VIRTIO_BLK_S_OK);
        //free the desc
        Ok(())
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
        printk!(
            "desc_addr: {:#x}, avail_addr: {:#x}, used_addr: {:#x}\n",
            desc_addr,
            avail_addr,
            used_addr
        );
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
        let mut features: u32 = 0;
        let mut status: u32 = 0;
        let mut device = Box::new(VirtioBlkMMIODeivce {
            base,
            size,
            sector_size: 512,
            vq: Virtq::init(),
            cfg: None,
        });
        device.check_magic()?;
        if device.read_volatile::<u32>(MMIO_DEVICE_ID_OFFSET) != DEVICE_ID_BLOCK {
            return Err("the device is not a block device.");
        }
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
        // features &= !(1 << VIRTIO_F_NOTIFICATION_DATA);
        // features &= !(1 << VIRTIO_F_RING_PACKED);
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
        } else if version == VIRTIO_BLK_MODERN_VERSION {
            device.queue_init()?;
        } else {
            return Err("the version is not supported.");
        }
        status |= DRIVER_OK;
        device.write_volatile::<u32>(MMIO_STATUS_OFFSET, status);
        Ok(device)
    }
}
