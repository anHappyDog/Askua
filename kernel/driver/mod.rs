use core::ops::Add;

pub mod clint;
pub mod fdt;
pub mod plic;
pub mod rtc;
pub mod uart;
pub mod virtio;

pub trait Device {
    fn read_volatile<T>(&self, offset: usize) -> T
    where
        T: Add;
    fn write_volatile<T>(&self, offset: usize, value: T);
}

pub fn dev_init() {}
