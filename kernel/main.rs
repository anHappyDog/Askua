#![no_std]
#![no_main]
mod driver;
mod log;
mod virtio;
mod lock;

#[no_mangle]
pub extern "C"  fn _init() {
    printk!("Hello,askua.");
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
