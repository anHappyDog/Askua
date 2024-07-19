#![no_std]
#![no_main]
mod arch;
mod driver;
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
pub extern "C" fn _init(hartid: usize, dtb: usize) -> ! {
    printk!("{} has been selected to be the master core.", hartid);
    use alloc::vec::Vec;
    let mut t1 = Vec::new();
    t1.push(1);
    t1.push(1);
    t1.push(1);
    t1.push(1);
    t1.push(1);
    for i in t1.iter() {
        printk!("{}", i);
    }
    loop {}
}
