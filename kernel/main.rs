#![cfg_attr(debug_assertions, allow(unused))]
#![no_std]
#![no_main]
#![feature(asm_const)]

use driver::virtio::{blk::mmio::VirtioBlkMMIODeivce, VirtioMMIODeivce};
mod arch;
mod driver;
mod errno;
mod fs;
mod lock;
mod log;
mod mm;
mod proc;
mod smp;
mod sys;
mod trap;

extern crate alloc;

#[no_mangle]
pub extern "C" fn _init(hartid: usize, dtb: *const u8) -> ! {
    printk!("{} has been selected to be the master core.\n", hartid);

    loop {}
}
