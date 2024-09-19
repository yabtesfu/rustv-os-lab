$kernel = "target/riscv64gc-unknown-none-elf/debug/rustv-os-lab"
qemu-system-riscv64 `
  -machine virt `
  -cpu rv64 `
  -smp 4 `
  -m 128M `
  -nographic `
  -serial mon:stdio `
  -bios none `
  -kernel $kernel

