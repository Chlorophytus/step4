#![no_main]
#![no_std]

use core::panic::PanicInfo;
mod step4;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

fn _start() -> () {
    loop {}
}
