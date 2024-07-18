mod boot_param;
mod dir_entry;
mod superblock;

use super::Fs;

pub struct Fat32Fs;

impl Fs for Fat32Fs {}
