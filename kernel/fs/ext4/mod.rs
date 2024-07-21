mod crc;
mod crypto;
mod extent;
mod feature;
mod group;
mod inode;
mod journal;
mod mmp;
mod superblock;
mod xattr;

use core::error::Error;

use alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc};
use inode::Ext4Inode;
use superblock::Ext4SuperBlock;

use crate::{fs::buffer, lock::irq_safe::spin::IrqSafeSpinlock};

use super::{buffer::Buffer, vfs::dentry::Dentry, Fs, FsDev};

pub(crate) struct Ext4Fs {
    superblock: IrqSafeSpinlock<Box<Ext4SuperBlock>>,
    dev: FsDev,
    buffer_list: IrqSafeSpinlock<BTreeMap<usize, Arc<IrqSafeSpinlock<Buffer>>>>,
    root_inode: Arc<IrqSafeSpinlock<Ext4Inode>>,
    inode_list: IrqSafeSpinlock<BTreeMap<usize, Arc<IrqSafeSpinlock<Ext4Inode>>>>,
    dentry_list: IrqSafeSpinlock<BTreeMap<String, Arc<IrqSafeSpinlock<Dentry>>>>,
    group_desc_list:
        IrqSafeSpinlock<BTreeMap<usize, Arc<IrqSafeSpinlock<Box<group::Ext4GroupDesc>>>>>,
    inode_bitmap: IrqSafeSpinlock<BTreeMap<usize, Arc<IrqSafeSpinlock<Buffer>>>>,
}

impl Fs for Ext4Fs {}

impl Ext4Fs {
    pub fn load(dev: FsDev) -> Result<Box<Self>, Box<dyn Error>> {
        let inode_list =
            IrqSafeSpinlock::new(BTreeMap::<usize, Arc<IrqSafeSpinlock<Ext4Inode>>>::new());
        let dentry_list =
            IrqSafeSpinlock::new(BTreeMap::<String, Arc<IrqSafeSpinlock<Dentry>>>::new());
        let buffer_list =
            IrqSafeSpinlock::new(BTreeMap::<usize, Arc<IrqSafeSpinlock<Buffer>>>::new());
        let superblock = IrqSafeSpinlock::new(Ext4SuperBlock::empty());
        let root_inode = Arc::new(IrqSafeSpinlock::new(*Ext4Inode::empty()));
        let group_desc_list = IrqSafeSpinlock::new(BTreeMap::<usize, Arc<IrqSafeSpinlock<Box<group::Ext4GroupDesc>>>>::new());
        let inode_bitmap = IrqSafeSpinlock::new(BTreeMap::<usize, Arc<IrqSafeSpinlock<Buffer>>>::new());
        let mut ext4fs = Box::new(Self {
            superblock,
            dev,
            buffer_list,
            root_inode,
            inode_list,
            dentry_list,
            group_desc_list,
            inode_bitmap,
        });
        // after create an "empty" ext4 fs, we now should use the dev to load its content.
        // ext4fs.load_superblock()?;
        // ext4fs.load_group_desc()?;
        // ext4fs.load_inode_bitmap()?;
        // ext4fs.load_root_inode()?;
        Ok(ext4fs)
    }
}
