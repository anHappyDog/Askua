use core::sync::atomic::AtomicU32;

use alloc::sync::Arc;

use crate::{
    fs::vfs::{dentry::Dentry, file::File},
    lock::irq_safe::spin::IrqSafeSpinlock,
};

use super::inode::Ext4Inode;

pub struct Ext4File {
    f_dentry: Arc<IrqSafeSpinlock<Dentry>>,
    f_inode: Arc<IrqSafeSpinlock<Ext4Inode>>,
    f_count: AtomicU32,
    f_pos: usize,
    f_flags: u32,
}

impl File for Ext4File {}
