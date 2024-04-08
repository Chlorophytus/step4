// Debugger UART, etc.

#[cfg(s4hwtype = "zynq")]
pub fn setup() {
    let uart_base = env!("S4UARTADDRESS").parse::<u32>().unwrap() as *mut u32;
    unsafe {
        // Disable the UART before set-up
        core::ptr::write_volatile::<u32>(uart_base, 0x0000_0000);

        // Baud rate divider for 115200
        core::ptr::write_volatile::<u32>(uart_base.byte_add(0x34) as *mut u32, 6);
        // Baud rate generator for 115200
        core::ptr::write_volatile::<u32>(uart_base.byte_add(0x18), 62);

        // 8N1 bit mode
        core::ptr::write_volatile::<u32>(uart_base.byte_add(0x04), (1 << 5));

        // Enable the UART with receiver-transmitter
        core::ptr::write_volatile::<u32>(uart_base, (1 << 4) | (1 << 2) | (1 << 1) | (1 << 0));
    }
}

#[cfg(s4hwtype = "zynq")]
pub fn put_char(character: u8) {
    let uart_base = env!("S4UARTADDRESS").parse::<u32>().unwrap() as *mut u32;
    unsafe {
        while (core::ptr::read_volatile(uart_base.byte_add(0x2C)) & (1 << 4)) > 0 {}
        core::ptr::write_volatile::<u32>(uart_base.byte_add(0x30), character as u32);
    }
}

pub fn put_str(text: &str) {
    for c in text.as_bytes() {
        put_char(*c);
    }
}
