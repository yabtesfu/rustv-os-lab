# QEMU virt Memory Map

The RISC-V `virt` machine starts DRAM at `0x8000_0000`, which is where this kernel is linked.

| Region | Base | Notes |
|---|---:|---|
| CLINT | `0x0200_0000` | timer and software interrupts |
| PLIC | `0x0c00_0000` | external interrupt controller |
| UART0 | `0x1000_0000` | NS16550a console |
| VIRTIO | `0x1000_1000` | virtio MMIO devices |
| DRAM | `0x8000_0000` | kernel image, stack, heap |

The linker script exports `_text_start`, `_bss_start`, `_heap_start`, and `_memory_end` so the kernel can reason about its own layout.

