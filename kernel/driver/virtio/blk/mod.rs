use core::ops::Add;

pub mod mmio;
pub mod pci;

use super::VirtioDevice;
use crate::driver::Device;

struct VirtioBlkDeivce {
    base: usize,
    size: usize,
}

impl VirtioDevice for VirtioBlkDeivce {}
impl Device for VirtioBlkDeivce {
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

pub(self) const VIRTIO_BLK_T_IN: usize = 0;
pub(self) const VIRTIO_BLK_T_OUT: usize = 1;
pub(self) const VIRTIO_BLK_T_FLUSH: usize = 4;
pub(self) const VIRTIO_BLK_T_GET_ID: usize = 8;
pub(self) const VIRTIO_BLK_T_GET_LIFETIME: usize = 10;
pub(self) const VIRTIO_BLK_T_DISCARD: usize = 11;
pub(self) const VIRTIO_BLK_T_WRITE_ZEROES: usize = 13;
pub(self) const VIRTIO_BLK_T_SECURE_ERASE: usize = 14;

pub(self) struct VirtioBlkGeometry {
    cylinders: u16,
    heads: u8,
    sectors: u8,
}

pub(self) struct VirtioBlkTopology {
    physical_block_exp: u8,
    alignment_offset: u8,
    min_io_size: u16,
    opt_io_size: u32,
}

pub(self) struct VirtioBlkV0Config {
    capacity: u64,
    size_max: u32,
    seg_max: u32,
    geometry: VirtioBlkGeometry,
    blk_size: u32,
    topology: VirtioBlkTopology,
    max_discard_sectors: u32,
    discard_sector_alignment: u32,
    max_discard_seg: u32,
    max_write_zeroes_sectors: u32,
    max_write_zeroes_seg: u32,
    write_zeroes_may_unmap: u8,
    unused: [u8; 3],
    max_secure_erase_sectors: u32,
    max_secure_erase_seg: u32,
    secure_erase_sector_alignment: u32,
}

pub(self) struct VirtioBlkV1Config {}

pub(self) enum VirtioBlkConfig {
    V0(VirtioBlkV0Config),
    V1(VirtioBlkV1Config),
}
