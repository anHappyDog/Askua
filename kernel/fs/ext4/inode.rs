use alloc::boxed::Box;

use crate::fs::vfs::inode::Inode;

pub(super) struct Ext4Inode {
    i_mode: u16,
    i_uid: u16,
    i_size_lo: u32,
    i_atime: u32,
    i_ctime: u32,
    i_mtime: u32,
    i_dtime: u32,
    i_gid: u16,
    i_links_count: u16,
    i_blocks_lo: u32,
    i_flags: u32,
    i_osd1: u32,
    i_block: [u32; 15],
    i_generation: u32,
    i_file_acl: u32,
    i_size_high: u32,
    i_obso_faddr: u32,
    i_osd2: [u32; 3],
    i_extra_size: u16,
    i_checksum_hi: u16,
    i_ctime_extra: u32,
    i_mtime_extra: u32,
    i_atime_extra: u32,
    i_crtime: u32,
    i_crtime_extra: u32,
    i_version_hi: u32,
    i_projid: u32,
}

impl Inode for Ext4Inode {}

impl Ext4Inode {
    pub(super) fn empty() -> Box<Self> {
        todo!("return an empty inode.")
    }
}
