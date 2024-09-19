TARGET := riscv64gc-unknown-none-elf
KERNEL := target/$(TARGET)/debug/rustv-os-lab
QEMU := qemu-system-riscv64

.PHONY: build run debug check clean

build:
	cargo build

run: build
	$(QEMU) -machine virt -cpu rv64 -smp 4 -m 128M -nographic -serial mon:stdio -bios none -kernel $(KERNEL)

debug: build
	$(QEMU) -machine virt -cpu rv64 -smp 4 -m 128M -nographic -serial mon:stdio -bios none -S -s -kernel $(KERNEL)

check:
	cargo check

clean:
	cargo clean

