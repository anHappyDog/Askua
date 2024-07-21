use alloc::sync::Arc;

use crate::lock::irq_safe::spin::IrqSafeSpinlock;

use super::Fs;

const BLOCK_SIZE: usize = 4096;

bitflags::bitflags! {
    struct BufferFlags : u32 {
        const DIRTY = 0b00000001;
        const VALID = 0b00000010;
    }
}

pub(super) struct Buffer {
    data: [u8; BLOCK_SIZE],
    sector: usize,
    flags: BufferFlags,
    fs: Arc<IrqSafeSpinlock<dyn Fs>>,
}

impl Buffer {
    pub fn create(
        sector: usize,
        fs: Arc<IrqSafeSpinlock<dyn Fs>>,
    ) -> Result<Arc<IrqSafeSpinlock<Self>>, &'static str> {
        let mut res = Arc::new(IrqSafeSpinlock::new(Self {
            data: [0; BLOCK_SIZE],
            sector,
            flags: BufferFlags::VALID,
            fs,
        }));
        let locked_res = res.lock();

        drop(locked_res);
        Ok(res)
    }
    pub fn write(&mut self, data: &[u8], offset: usize) -> Result<(), &'static str> {
        todo!("Buffer::write")
    }
    pub fn read(&self, data: &mut [u8], offset: usize) -> Result<(), &'static str> {
        todo!("Buffer::read")
    }
}
