#![no_std]
#![no_main]
mod driver;
mod log;
mod lock;
mod mm;
mod fs;
mod trap;
mod arch;

#[no_mangle]
pub extern "C"  fn _init() {
    printk!("Hello,askua.");
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
