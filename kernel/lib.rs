#![no_std]

#[no_mangle]
extern "C" fn _init() {
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
