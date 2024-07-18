use core::ops::Add;

use super::Plic;
use crate::driver::Device;

const SIFIVE_BASE_ADDR: usize = 0x0c0000000;
const SIFIVE_INT_PRI_BASE: usize = 0x0;
const SIFIVE_INT_PENDING_BASE: usize = 0x1000;
const SIFIVE_INT_ENABLE_BASE: usize = 0x2000;
const SIFIVE_INT_PRI_THRESHOLD_BASE: usize = 0x200000;
const SIFIVE_INT_PRI_CLAIM_BASE: usize = 0x200004;

const SIFIVE_INT_NUM: usize = 1024;
const SIFIVE_ENABLE_CONTEXT_BLOCK_SIZE: usize = 0x80;
const SIFIVE_BASE_SIZE: usize = 0x600000;

const VIRTIO_MMIO_INT: usize = 0x1;
const GOLD_FISH_RTC_INT: usize = 0xb;

pub struct SifivePlic {
    base: usize,
    size: usize,
}

impl Plic for SifivePlic {
    fn init(base: usize, size: usize) -> Self {
        SifivePlic { base, size }
    }

    fn claim(&self, context: usize) -> u32 {
        self.read_volatile(SIFIVE_INT_PRI_CLAIM_BASE + 0x1000 * context)
    }

    fn complete(&self, context: usize, irq: u32) {
        self.write_volatile(SIFIVE_INT_PRI_CLAIM_BASE + 0x1000 * context, irq);
    }
}
impl Device for SifivePlic {
    fn read_volatile<T>(&self, offset: usize) -> T
    where
        T: Add,
    {
        unsafe { ((self.base + offset) as *const T).read_volatile() }
    }
    fn write_volatile<T>(&self, offset: usize, value: T) {
        unsafe {
            ((self.base + offset) as *mut T).write_volatile(value);
        }
    }
}
