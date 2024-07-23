.globl _armv7a_setup
.globl _armv7a_check_timer_a9

_armv7a_setup:
    // The Cortex-A9 should be in Supervisor mode in order to proceed
    // SEE: Arm DDI0388G page identifier 3-6
    // Move the Program Status Register to R0
    mrs r1, cpsr

    // Bitwise unmask all bits except for mode bits
    and r1, r1, #0x1F

    // Bitmask supervisor mode check
    mov r0, #0x13

    // Check if current mode is supervisor
    cmp r0, r1

    // return
    bx lr

// TODO: Create Armv7-A interrupt timer for Zynq
_armv7a_check_timer_a9:
    // https://wiki.osdev.org/ARM_Paging#ARMv7-A
    // There is a typo, "mcr" should be "mrc" I think
    mrc p15, 0, r0, c0, c1, 4

    // check what type of pager we got
    and r0, r0, #0xF

    // return
    bx lr
