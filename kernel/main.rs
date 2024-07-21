#![cfg_attr(debug_assertions, allow(unused))]
#![no_std]
#![no_main]
#![feature(asm_const)]

use alloc::{boxed::Box, sync::Arc};
use driver::virtio::{
    self,
    blk::{mmio::VirtioBlkMMIODeivce, VirtioBlkDevice},
    mmio::VirtioMMIODeivce,
    net::mmio::VirtioNetMMIODeivce,
};
use lock::irq_safe::spin::IrqSafeSpinlock;
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
    // driver::dev_init();
    // mm::init();
    let virtio_mmio1: Arc<IrqSafeSpinlock<Box<dyn VirtioBlkDevice>>> =
        Arc::new(IrqSafeSpinlock::new(
            virtio::blk::mmio::VirtioBlkMMIODeivce::mmio_init(0x10001000, 0x1000).expect("sa"),
        ));
    let ext4fs = fs::ext4::Ext4Fs::load(fs::FsDev::VirtioBlk(virtio_mmio1)).expect("sa");
    printk!("ext4fs loaded.\n");
    trap::init();
    // proc::init();
    // fs::init();
    // then start to schedule

    crate::arch::rv64::sbi::sbi_shutdown();
}
