// Debugger UART, etc.

#[cfg(s4uart = "zynq")]
pub fn setup() {
    unsafe {
        // Disable the UART before set-up
        core::ptr::write_volatile::<u32>(0xE000_0000 as *mut u32, 0x0000_0000);

        // Baud rate divider for 115200
        core::ptr::write_volatile::<u32>(0xE000_0034 as *mut u32, 6);
        // Baud rate generator for 115200
        core::ptr::write_volatile::<u32>(0xE000_0018 as *mut u32, 62);

        // 8N1 bit mode
        core::ptr::write_volatile::<u32>(0xE000_0004 as *mut u32, (1 << 5));

        // Enable the UART with receiver-transmitter
        core::ptr::write_volatile::<u32>(
            0xE000_0000 as *mut u32,
            (1 << 4) | (1 << 2) | (1 << 1) | (1 << 0),
        );
    }
}

#[cfg(s4uart = "zynq")]
pub fn put_char(character: u8) {
    unsafe {
        while (core::ptr::read_volatile(0xE000_002C as *mut u32) & (1 << 4)) > 0 {}
        core::ptr::write_volatile::<u32>(0xE000_0030 as *mut u32, character as u32);
    }
}

pub fn put_str(text: &str) {
    for c in text.as_bytes() {
        put_char(*c);
    }
}