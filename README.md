# step4

A work-in-progress L4 microkernel implementation in Rust. 

## Example emulation

On AMD Zynq-7000 platforms:

```
cargo build --target armv7a-none-eabi
qemu-system-arm -s -S -machine xilinx-zynq-a9 -m 512M -serial mon:stdio -cpu cortex-a9 -kernel ./target/armv7a-none-eabi/debug/step4
```

Then attach GDB.
