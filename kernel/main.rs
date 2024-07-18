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
mod trap;
mod sys;
#[no_mangle]
pub extern "C" fn _init() {
    printk!("Hello,askua.");
    loop {}
}
