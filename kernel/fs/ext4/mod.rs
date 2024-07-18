mod crc;
mod crypto;
mod extent;
mod feature;
mod inode;
mod journal;
mod mmp;
mod superblock;
mod xattr;

use super::Fs;

struct Ext4Fs;

impl Fs for Ext4Fs {}
