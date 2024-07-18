use core::ops::Add;

use super::VirtioDevice;
use crate::driver::Device;

struct VirtioBlkDeivce {
    base: usize,
    size: usize,
}

impl VirtioDevice for VirtioBlkDeivce {}
impl Device for VirtioBlkDeivce {
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
