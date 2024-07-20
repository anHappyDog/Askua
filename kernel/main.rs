#![cfg_attr(debug_assertions, allow(unused))]
#![no_std]
#![no_main]

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
    use driver::fdt::FdtHeader;
    let fdt_tree = driver::fdt::FdtTree::from_bytes(dtb).expect("parse fdt tree failed.");
    printk!("parse fdt tree finished.\n");
    let virtio_mmio1 = VirtioBlkMMIODeivce::mmio_init(0x10001000, 0x1000).expect("init the virtio mmio blk device failed.");
    printk!("virtio mmio blk device init finished.\n");
    loop {}
}
