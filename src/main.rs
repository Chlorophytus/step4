#![no_main]
#![no_std]

use core::{arch::global_asm, fmt::Write, panic::PanicInfo};
use heapless::String;
use step4::kernel::{Description, Kernel};
mod step4;

use crate::step4::{debug, kernel};

#[cfg(s4arch = "armv7a")]
extern "C" {
    fn _armv7a_setup() -> u32;
}

#[cfg(s4arch = "armv7a")]
global_asm!(
    "
    .section .vector_table, \"ax\", %progbits
    b _start // TODO: Reset
    b _start // TODO: UndefinedInstruction
    b _start // TODO: SupervisorCall
    b _start // TODO: PrefetchAbort
    b _start // TODO: DataAbort
    b _start // TODO: HypervisorTrap
    b _start // TODO: IRQInterrupt
    b _start // TODO: FIQInterrupt
    "
);

#[cfg(s4arch = "armv7a")]
unsafe fn _setup() -> bool {
    _armv7a_setup() == 0x13
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    debug::put_str("*** START PANIC ***\r\n");
    if let Some(location) = _panic.location() {
        let mut info = String::<1024>::new();
        if let Ok(()) = write!(
            info,
            "Source information: '{}' (line {})\r\n",
            location.file(),
            location.line()
        ) {
            debug::put_str(info.as_str());
        } else {
            debug::put_str("No source information available.");
        }
    }
    debug::put_str("**** END PANIC ****\r\n");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    debug::setup();
    debug::put_str("Step4 microkernel\r\n");

    if unsafe { _setup() } {
        debug::put_str("... Setup okay\r\n");
    } else {
        debug::put_str("... Setup fail, panicing\r\n");
        panic!();
    }

    // let config = kernel::Configuration {
    //     big_endian: false,
    // };
    // let description = kernel::Description::new(config);
    // let kernel = kernel::Kernel::new(config);
    let kernel_destination = 0x00100000 as *mut Kernel;
    let kernel = Kernel::allocate(kernel_destination, 0, 0x1000);

    debug::put_str("... Alright\r\n");

    loop {}
}
