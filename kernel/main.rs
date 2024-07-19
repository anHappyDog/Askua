#![cfg_attr(debug_assertions, allow(unused))]
#![no_std]
#![no_main]
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
    printk!("{} has been selected to be the master core.", hartid);
    use driver::fdt::FdtHeader;
    unsafe {
        let tt = core::slice::from_raw_parts(dtb, 40) as &'static [u8];
        printk!("start to parse the fdt tree.");
        let fdt_tree = driver::fdt::FdtTree::from_bytes(tt).expect("parse fdt tree failed.");
    }
    printk!("parse fdt tree finished.");
    loop {}
}
