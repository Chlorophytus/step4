use crate::step4::{debug, kernel};
use core::{fmt::Write, mem::size_of};
use heapless::String;

pub struct MemoryInfo {
    first_descriptor: u16,
    num_descriptors: u16,
}
pub struct Description {
    id: u32,
    generation_date: u32,
    version: u32,
    supplier: u32,
}

impl MemoryInfo {
    pub fn get(&self) -> u32 {
        ((self.first_descriptor as u32) << 16) | ((self.num_descriptors as u32) << 0)
    }
    pub fn put_count(&mut self, descriptors: u16) {
        self.num_descriptors = descriptors
    }
}

impl Description {
    pub fn new() -> Self {
        Self {
            id: u32::from_be_bytes([0x54, 0x01, 0x00, 0x00]),
            generation_date: env!("S4BUILDTIME").parse::<u32>().unwrap(),
            version: env!("S4BUILDVERSION").parse::<u32>().unwrap(),
            supplier: u32::from_be_bytes([b'R', b'J', b'M', 0]),
        }
    }
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
    // space_control: u32,
    // thread_control: u32,
    // processor_control: u32,
    // memory_control: u32,

    // ipc: u32,
    // lipc: u32,
    // unmap: u32,
    // exchange_registers: u32,

    // system_clock: u32,
    // thread_switch: u32,
    // schedule: u32,
    // undefined_map3: u32,
}

impl Kernel {
    pub fn allocate(destination: *mut Kernel, boot_info: u32, offset_memory: u16) {
        let memory = MemoryInfo {
            first_descriptor: offset_memory,
            num_descriptors: 1,
        };
        let description = Description::new();

        let ptr_description = &description as *const Description;
        let ptr_memory = &memory as *const MemoryInfo;

        let ptr_kernel_memory = destination as *mut MemoryInfo;
        let ptr_kernel_description = destination as *mut Description;

        let dest_memory = unsafe { ptr_kernel_memory.byte_add(offset_memory as usize) };
        let dest_description = unsafe { ptr_kernel_description.add(size_of::<Kernel>()) };

        let my_kernel = Self {
            l4_word: u32::from_be_bytes([b'L', b'4', 230, b'K']),
            api_version: u32::from_be_bytes([0x90, 0x00, 0x00, 0x00]),
            api_flags: u32::from_be_bytes([0x00, 0x00, 0x00, 0b00000000]),
            kern_desc_ptr: dest_description as u32,
            undefined_map0: [0; 17],
            memory_info: memory.get(),
            undefined_map1: [0; 24],
            utcb_info: 0, // TODO
            kip_area_info: 8,
            undefined_map2: [0; 2],
            boot_info: boot_info,
            proc_desc_ptr: 0, // TODO
            clock_info: 0,    // TODO
            thread_info: 0,   // TODO
            page_info: 0,
            processor_info: 0,
        };

        let ptr_kernel = &my_kernel as *const Kernel;

        let mut mem_str = String::<1024>::new();
        write!(
            mem_str,
            "... Copying memory info from {:#08x} into {:#08x}\r\n",
            ptr_memory as u32, dest_memory as u32
        );

        let mut desc_str = String::<1024>::new();
        write!(
            desc_str,
            "... Copying description info from {:#08x} into {:#08x}\r\n",
            ptr_description as u32, dest_description as u32
        );

        let mut kernel_str = String::<1024>::new();
        write!(
            kernel_str,
            "... Copying kernel info from {:#08x} into {:#08x}\r\n",
            ptr_kernel as u32, destination as u32
        );

        unsafe {
            debug::put_str(mem_str.as_str());
            core::ptr::copy(ptr_memory, dest_memory, size_of::<MemoryInfo>());
            debug::put_str(desc_str.as_str());
            core::ptr::copy(ptr_description, dest_description, size_of::<Description>());
            debug::put_str(kernel_str.as_str());
            core::ptr::copy(ptr_kernel, destination as *mut Kernel, size_of::<Kernel>())
        }
    }
}
