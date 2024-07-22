mod boot_param;
mod dir_entry;
mod superblock;
use super::Fs;

pub struct Fat32Fs {
    super_block: superblock::Fat32SuperBlock,
}

impl Fs for Fat32Fs {
    fn create_noexist_buffer(
        &self,
        sector: usize,
    ) -> Result<(), alloc::boxed::Box<dyn core::error::Error>> {
        todo!()
    }

    fn read_buffer(
        &self,
        data: &mut [u8],
        sector: usize,
        offset: usize,
    ) -> Result<(), alloc::boxed::Box<dyn core::error::Error>> {
        todo!()
    }

    fn write_buffer(
        &self,
        data: &[u8],
        sector: usize,
        offset: usize,
    ) -> Result<(), alloc::boxed::Box<dyn core::error::Error>> {
        todo!()
    }

    fn sync(&self) -> Result<(), alloc::boxed::Box<dyn core::error::Error>> {
        todo!()
    }
}
