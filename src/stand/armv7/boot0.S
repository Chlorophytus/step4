; ARMv7 vector table
.section .vector_table
b hang ; TODO: Reset
b hang ; TODO: UndefinedInstruction
b hang ; TODO: SupervisorCall
b hang ; TODO: PrefetchAbort
b hang ; TODO: DataAbort
b hang ; TODO: HypervisorTrap
b hang ; TODO: IRQInterrupt
b hang ; TODO: FIQInterrupt
.section .text
hang:
  b hang
