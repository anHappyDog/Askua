mod boot_param;
mod dir_entry;
mod superblock;
use super::Fs;

pub struct Fat32Fs {
    super_block: superblock::Fat32SuperBlock,
    
}

impl Fs for Fat32Fs {}
