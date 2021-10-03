pub type Type = u32;

pub enum RegularType {
    ExchangeRegisters(Type),
    SystemClock(Type),
    ThreadSwitch(Type),
    Schedule(Type),
    Ipc(Type),
    Lipc(Type),
    Unmap(Type),
}

pub enum SlowType {
    KernelInterface(Type),
}


pub enum PrivilegedType {
    ThreadControl(Type),
    SpaceControl(Type),
    ProcessorControl(Type),
    MemoryControl(Type),
}

trait SyscallTrait {
    fn syscall();
}

