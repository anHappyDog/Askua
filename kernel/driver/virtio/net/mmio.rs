use alloc::{boxed::Box, vec::Vec};

use crate::driver::{
    virtio::{
        mmio::{
            consts::{
                MMIO_DEVICE_FEATURES_SEL_OFFSET, MMIO_DEVICE_ID_OFFSET, MMIO_MAGIC_OFFSET,
                MMIO_QUEUE_NUM_MAX_OFFSET, MMIO_QUEUE_READY_OFFSET, MMIO_QUEUE_SEL_OFFSET,
                MMIO_STATUS_OFFSET, MMIO_VERSION_OFFSET, VIRTIO_BLK_LEGACY_VERSION,
                VIRTIO_BLK_MODERN_VERSION, VIRTIO_MMIO_MAGIC,
            },
            VirtioMMIODeivce,
        },
        virtq::{Virtq, VIRTQ_DESC_LIST_LENGTH},
        ACKNOWLEDGE, DRIVER, DRIVER_OK, FEATURE_OK,
    },
    Device,
};

use super::{VirtioNetConfig, VirtioNetDevice};

pub struct VirtioNetMMIODeivce {
    base: usize,
    size: usize,
    cfg: Option<VirtioNetConfig>,
    vqs: Vec<Virtq>,
}

impl Device for VirtioNetMMIODeivce {
    fn read_volatile<T>(&self, offset: usize) -> T
    where
        T: core::ops::Add,
    {
        todo!()
    }

    fn write_volatile<T>(&self, offset: usize, value: T) {
        todo!()
    }
}

const DEVICE_ID_NETWORK: u32 = 1;

impl VirtioNetDevice for VirtioNetMMIODeivce {
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), &'static str> {
        todo!()
    }

    fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        todo!()
    }
}

impl VirtioNetMMIODeivce {
    fn legacy_queue_init(&mut self) -> Result<(), &'static str> {
        todo!()
    }
    fn queue_init(&self) -> Result<(), &'static str> {
        todo!()
    }
    fn check_magic(&self) -> Result<(), &'static str> {
        let magic = self.read_volatile::<u32>(MMIO_MAGIC_OFFSET);
        if magic != VIRTIO_MMIO_MAGIC {
            return Err("the magic number is not correct.");
        }
        Ok(())
    }
}

impl VirtioMMIODeivce for VirtioNetMMIODeivce {
    fn mmio_init(base: usize, size: usize) -> Result<alloc::boxed::Box<Self>, &'static str> {
        let mut features: u32 = 0;
        let mut status: u32 = 0;
        let mut device = Box::new(VirtioNetMMIODeivce {
            base,
            size,
            cfg: None,
            vqs: Vec::new(),
        });
        device.check_magic()?;
        if device.read_volatile::<u32>(MMIO_DEVICE_ID_OFFSET) != DEVICE_ID_NETWORK {
            return Err("the device is not a block device.");
        }
        let version = device.read_volatile::<u32>(MMIO_VERSION_OFFSET);

        device.write_volatile::<u32>(MMIO_STATUS_OFFSET, status);
        status |= ACKNOWLEDGE;
        device.write_volatile::<u32>(MMIO_STATUS_OFFSET, status);
        status |= DRIVER;
        device.write_volatile::<u32>(MMIO_STATUS_OFFSET, status);
        device.write_volatile::<u32>(MMIO_DEVICE_FEATURES_SEL_OFFSET, 0);
        // features = device.read_volatile::<u32>(MMIO_DEVICE_FEATURES_OFFSET);
        // features &= !(1 << VIRTIO_BLK_F_RO);
        // features &= !(1 << VIRTIO_BLK_F_MQ);
        // features &= !(1 << VIRTIO_F_INDIRECT_DESC);
        // features &= !(1 << VIRTIO_F_EVENT_IDX);

        // device.write_volatile::<u32>(MMIO_DRIVER_FEATURES_OFFSET, features);
        // device.write_volatile::<u32>(MMIO_DEVICE_FEATURES_SEL_OFFSET, 1);
        // features = device.read_volatile::<u32>(MMIO_DEVICE_FEATURES_OFFSET);
        // // features &= !(1 << VIRTIO_F_NOTIFICATION_DATA);
        // // features &= !(1 << VIRTIO_F_RING_PACKED);
        // device.write_volatile::<u32>(MMIO_DRIVER_FEATURES_SEL_OFFSET, 1);
        // device.write_volatile::<u32>(MMIO_DRIVER_FEATURES_OFFSET, features);
        // status |= FEATURE_OK;
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
