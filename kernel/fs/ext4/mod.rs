mod crc;
mod crypto;
mod extent;
mod feature;
mod inode;
mod journal;
mod mmp;
mod superblock;
mod xattr;

use alloc::sync::Arc;
use superblock::Ext4SuperBlock;

use crate::lock::irq_safe::spin::IrqSafeSpinlock;

use super::{Fs, FsDev};

struct Ext4Fs {
    superblock: Arc<IrqSafeSpinlock<Ext4SuperBlock>>,
    dev: FsDev,
    
}

impl Fs for Ext4Fs {}
