.option norvc
.section .text.init
.global _start

_start:
    csrr    t0, mhartid
    bnez    t0, park_hart

    csrw    satp, zero

.option push
.option norelax
    la      gp, _global_pointer
.option pop

    la      sp, _stack_top

    la      a0, _bss_start
    la      a1, _bss_end
    bgeu    a0, a1, bss_done

clear_bss:
    sd      zero, 0(a0)
    addi    a0, a0, 8
    bltu    a0, a1, clear_bss

bss_done:
    csrr    a0, mhartid
    mv      a1, zero
    call    kmain

park_hart:
    wfi
    j       park_hart

