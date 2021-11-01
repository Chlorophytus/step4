pub enum RegularType {
    ExchangeRegisters(u32),
    SystemClock(u32),
    ThreadSwitch(u32),
    Schedule(u32),
    Ipc(u32),
    Lipc(u32),
    Unmap(u32),
}

pub enum SlowType {
    KernelInterface(u32),
}


pub enum PrivilegedType {
    ThreadControl(u32),
    SpaceControl(u32),
    ProcessorControl(u32),
    MemoryControl(u32),
}

trait SyscallTrait {
    fn syscall();
}

