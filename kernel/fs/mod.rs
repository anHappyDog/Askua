use alloc::{boxed::Box, sync::Arc};

use crate::{driver::virtio::blk::VirtioBlkDevice, lock::irq_safe::spin::IrqSafeSpinlock};

pub mod buffer;
pub mod ext2;
pub mod ext4;
pub mod fat32;
pub mod vfs;

pub(self) trait Fs {}

pub(crate) enum FsDev {
    VirtioBlk(Arc<IrqSafeSpinlock<Box<dyn VirtioBlkDevice>>>),
}

pub(super) fn init() {}
