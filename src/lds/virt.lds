OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY
{
  RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
  . = ORIGIN(RAM);
  PROVIDE(_memory_start = .);

  .text : ALIGN(4K) {
    PROVIDE(_text_start = .);
    *(.text.init)
    *(.text .text.*)
    PROVIDE(_text_end = .);
  } > RAM

  .rodata : ALIGN(4K) {
    PROVIDE(_rodata_start = .);
    *(.rodata .rodata.*)
    PROVIDE(_rodata_end = .);
  } > RAM

  .data : ALIGN(4K) {
    PROVIDE(_data_start = .);
    *(.sdata .sdata.*)
    *(.data .data.*)
    PROVIDE(_global_pointer = .);
    PROVIDE(_data_end = .);
  } > RAM

  .bss (NOLOAD) : ALIGN(4K) {
    PROVIDE(_bss_start = .);
    *(.sbss .sbss.*)
    *(.bss .bss.*)
    *(COMMON)
    PROVIDE(_bss_end = .);
  } > RAM

  . = ALIGN(4K);
  PROVIDE(_stack_start = .);
  . += 64K;
  PROVIDE(_stack_top = .);

  . = ALIGN(4K);
  PROVIDE(_heap_start = .);
  PROVIDE(_memory_end = ORIGIN(RAM) + LENGTH(RAM));
  PROVIDE(_heap_size = _memory_end - _heap_start);
}

