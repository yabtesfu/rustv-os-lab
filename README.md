# RustV OS Lab

RustV OS Lab is an educational RISC-V operating system prototype written in Rust. It follows the bare-metal learning path: take control at `0x8000_0000`, clear `.bss`, enter Rust, talk through the QEMU `virt` UART, model physical pages, sketch Sv39 page tables, route traps, and build a small scheduler/syscall layer.

The project is intentionally small enough to study, but structured like a real kernel lab rather than a single-file demo.

## Features

- RISC-V `virt` memory map notes and linker script
- Assembly boot path that parks secondary harts and clears `.bss`
- `no_std` Rust kernel entry point and panic handler
- NS16550a UART console with `print!` and `println!`
- MMIO helpers using volatile reads/writes
- Page-grained physical allocator model
- Sv39 page-table entry helpers and virtual-address decoding
- Trap cause decoding and trap frame model
- Cooperative task scheduler model
- Minimal syscall dispatcher
- QEMU, GDB, and build workflow documentation
- `push_project.sh` configured for the requested repository and date window

## Requirements

```bash
rustup install nightly
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils
```

You also need QEMU:

```bash
sudo apt install qemu-system-misc
```

## Build

```bash
cargo build
```

## Run

```bash
make run
```

## Layout

```txt
src/
  asm/boot.S          Boot hart selection and BSS clearing
  console/            UART-backed formatting macros
  drivers/            MMIO and NS16550a UART driver
  lds/virt.lds        QEMU virt linker script
  memory/             Page allocator, regions, Sv39 helpers
  riscv/              CSR wrappers and wait-for-interrupt helpers
  sync/               Spin lock primitive
  syscall/            Educational syscall model
  task/               Cooperative scheduler model
  trap/               Trap frame and trap cause decoding
```

## Roadmap

1. Boot one hart and park the rest.
2. Clear `.bss`, set the stack pointer, and jump into Rust.
3. Initialize UART output through MMIO.
4. Decode traps and page faults.
5. Manage physical pages with descriptors.
6. Build Sv39 mappings for kernel text, data, heap, and MMIO.
7. Add a timer tick and cooperative task scheduler.
8. Add user-mode experiments and syscall validation.
9. Explore virtio block and network devices.

This repository is a lab, not a production OS. The point is to make each kernel subsystem visible, readable, and easy to extend.

