# Kernel Roadmap

1. Boot one hart and park the rest.
2. Clear `.bss`, set the stack pointer, and jump into Rust.
3. Initialize UART output through MMIO.
4. Decode traps and page faults.
5. Manage physical pages with descriptors.
6. Build Sv39 mappings for kernel text, data, heap, and MMIO.
7. Add a timer tick and cooperative task scheduler.
8. Add user-mode experiments and syscall validation.
9. Explore virtio block and network devices.

