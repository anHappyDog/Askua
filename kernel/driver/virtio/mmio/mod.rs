pub mod consts;

use alloc::boxed::Box;

pub trait VirtioMMIODeivce {
    fn mmio_init(base: usize, size: usize) -> Result<Box<Self>, &'static str>;
}