use core::ops::Add;

use crate::driver::Device;

pub struct Ns16550a {
    base: usize,
    size: usize,
}

impl super::Uart for Ns16550a {
    fn getc(&self) -> u32 {
        unsafe { (self.base as *const u8).read_volatile() as u32 }
    }
    fn init(base: usize, size: usize) -> Self {
        Self { base, size }
    }
    fn putc(&self, c: u32) {
        unsafe {
            (self.base as *mut u8).write_volatile(c as u8);
        }
    }
}

impl Device for Ns16550a {
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
