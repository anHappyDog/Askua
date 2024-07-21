#![cfg_attr(debug_assertions, allow(unused))]
#![no_std]
#![no_main]
#![feature(asm_const)]

use driver::virtio::{
    blk::mmio::VirtioBlkMMIODeivce, mmio::VirtioMMIODeivce, net::mmio::VirtioNetMMIODeivce,
};
mod arch;
mod driver;
mod errno;
mod fs;
mod klib;
mod lock;
mod log;
mod mm;
mod proc;
mod smp;
mod trap;

extern crate alloc;

// 1. parse device tree
// 2. mount the support devices like uart, plic, virtio.
// 3. initialize the memory management
// 4. initialize the trap handling
// 5. initialize the process management
// 6. initialize the file system

#[no_mangle]
pub extern "C" fn _init(hartid: usize, dtb: *const u8) -> ! {
    printk!("{} has been selected to be the master core.\n", hartid);
    mm::init();
    trap::init();
    proc::init();
    fs::init();
    // then start to schedule

    crate::arch::rv64::sbi::sbi_shutdown();
}
