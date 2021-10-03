pub trait WxorXGuarantee<T> {
    const W_XOR_X_BITFIELD: T;
    const INTIAL_DATA: T;

    fn page(&self) -> T;
}

pub struct Kernel {
    l4_word: u32,
    api_version: u32,
    api_flags: u32,
    kern_desc_ptr: u32,
    
    undefined_map0: [u32; 17],
    memory_info: u32,
    
    undefined_map1: [u32; 24],
    
    utcb_info: u32,
    kip_area_info: u32,

    undefined_map2: [u32; 2],

    boot_info: u32,
    proc_desc_ptr: u32,

    clock_info: u32,
    thread_info: u32,
    page_info: u32,
    processor_info: u32,

    space_control: u32,
    thread_control: u32,
    processor_control: u32,
    memory_control: u32,

    ipc: u32,
    lipc: u32,
    unmap: u32,
    exchange_registers: u32,

    system_clock: u32,
    thread_switch: u32,
    schedule: u32,
    undefined_map3: u32,
}
/*
const KERNEL_INITIAL_MEMORY: &'static [] = &[
    // 0x00
    076_u8,
    052_u8,
    230_u8,
    075_u8,

    // 0x04
    0x84_u8,
    0x07_u8,
    UNDEFINED,
    UNDEFINED,

    // 0x08
    0b00000000_u8,
    UNDEFINED,
    UNDEFINED,
    UNDEFINED,

    // 0x0C
    UNDEFINED,
    UNDEFINED,
    UNDEFINED,
    UNDEFINED,
]
*/
