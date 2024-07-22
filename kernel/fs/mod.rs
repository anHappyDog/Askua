use core::error::Error;

use alloc::{boxed::Box, sync::Arc};
use buffer::Buffer;

use crate::{driver::virtio::blk::VirtioBlkDevice, lock::irq_safe::spin::IrqSafeSpinlock};

pub mod buffer;
pub mod ext2;
pub mod ext4;
pub mod fat32;
pub mod vfs;

pub(self) trait Fs {
    fn create_noexist_buffer(&self, sector: usize) -> Result<(), Box<dyn Error>>;
    fn read_buffer(
        &self,
        data: &mut [u8],
        sector: usize,
        offset: usize,
    ) -> Result<(), Box<dyn Error>>;
    fn write_buffer(&self, data: &[u8], sector: usize, offset: usize)
        -> Result<(), Box<dyn Error>>;
    fn sync(&self) -> Result<(), Box<dyn Error>>;
}

pub(crate) enum FsDev {
    VirtioBlk(Arc<IrqSafeSpinlock<Box<dyn VirtioBlkDevice>>>),
}

pub(super) fn init() {}
