pub mod buffer;
pub mod ext2;
pub mod ext4;
pub mod fat32;
pub mod vfs;

pub(self) trait Fs {}

pub(super) fn init() {}
