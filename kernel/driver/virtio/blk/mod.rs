use core::ops::Add;

pub mod mmio;
pub mod pci;

use super::VirtioDevice;
use crate::driver::Device;

const DEVICE_ID_BLOCK: u32 = 2;

pub trait VirtioBlkDevice {
    fn read_sectors(&mut self, data: &mut [u8], sector: u64, count: u32) -> Result<(), &'static str>;
    fn write_sectors(&mut self, data: &[u8], sector: u64, count: u32) -> Result<(), &'static str>;
}

#[repr(C)]
pub(self) struct VirtioBlkGeometry {
    cylinders: u16,
    heads: u8,
    sectors: u8,
}
#[repr(C)]
pub(self) struct VirtioBlkTopology {
    physical_block_exp: u8,
    alignment_offset: u8,
    min_io_size: u16,
    opt_io_size: u32,
}

#[repr(C)]
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
    zoned: VirtioBlkZonedCharacteristics,
}

pub(self) struct VirtioBlkV1Config {}

pub(self) enum VirtioBlkConfig {
    V0(VirtioBlkV0Config),
    V1(VirtioBlkV1Config),
}

const VIRTIO_BLK_F_SIZE_MAX: u32 = 1;
const VIRTIO_BLK_F_SEG_MAX: u32 = 2;
const VIRITO_BLK_F_GEOMETRY: u32 = 4;
const VIRTIO_BLK_F_RO: u32 = 5;
const VIRTIO_BLK_F_BLK_SIZE: u32 = 6;
const VIRTIO_BLK_F_FLUSH: u32 = 9;
const VIRTIO_BLK_F_TOPOLOGY: u32 = 10;
const VIRTIO_BLK_F_CONFIG_WCE: u32 = 11;
const VIRTIO_BLK_F_MQ: u32 = 12;
const VIRTIO_BLK_F_DISCARD: u32 = 13;
const VIRTIO_BLK_F_WRITE_ZEROES: u32 = 14;
const VIRTIO_BLK_F_LIFETIME: u32 = 15;
const VIRTIO_BLK_F_SECURE_ERASE: u32 = 16;
const VIRTIO_BLK_F_ZONED: u32 = 17;
#[repr(C)]
struct VirtioBlkZonedCharacteristics {
    zone_sectors: u32,
    max_open_zones: u32,
    max_active_zones: u32,
    max_append_sectors: u32,
    write_granularity: u32,
    model: u8,
    unused2: [u8; 3],
}
#[repr(C)]
struct VirtqBlkReq<'a> {
    type_: u32,
    reserved: u32,
    sector: u64,
    data: &'a [u8],
    status: u8,
}

// virtio_blk_req type
pub(self) const VIRTIO_BLK_T_IN: usize = 0;
pub(self) const VIRTIO_BLK_T_OUT: usize = 1;
pub(self) const VIRTIO_BLK_T_FLUSH: usize = 4;
pub(self) const VIRTIO_BLK_T_GET_ID: usize = 8;
pub(self) const VIRTIO_BLK_T_GET_LIFETIME: usize = 10;
pub(self) const VIRTIO_BLK_T_DISCARD: usize = 11;
pub(self) const VIRTIO_BLK_T_WRITE_ZEROES: usize = 13;
pub(self) const VIRTIO_BLK_T_SECURE_ERASE: usize = 14;
