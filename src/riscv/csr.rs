#[cfg(target_arch = "riscv64")]
#[inline]
pub fn read_scause() -> usize {
    let value;
    unsafe {
        core::arch::asm!("csrr {}, scause", out(reg) value, options(nomem, nostack));
    }
    value
}

#[cfg(not(target_arch = "riscv64"))]
#[inline]
pub fn read_scause() -> usize {
    0
}

#[cfg(target_arch = "riscv64")]
#[inline]
pub fn read_sepc() -> usize {
    let value;
    unsafe {
        core::arch::asm!("csrr {}, sepc", out(reg) value, options(nomem, nostack));
    }
    value
}

#[cfg(not(target_arch = "riscv64"))]
#[inline]
pub fn read_sepc() -> usize {
    0
}

#[cfg(target_arch = "riscv64")]
#[inline]
pub unsafe fn write_stvec(value: usize) {
    core::arch::asm!("csrw stvec, {}", in(reg) value, options(nomem, nostack));
}

#[cfg(not(target_arch = "riscv64"))]
#[inline]
pub unsafe fn write_stvec(_value: usize) {}

#[cfg(target_arch = "riscv64")]
#[inline]
pub unsafe fn write_satp(value: usize) {
    core::arch::asm!("csrw satp, {}", in(reg) value, options(nomem, nostack));
    core::arch::asm!("sfence.vma", options(nomem, nostack));
}

#[cfg(not(target_arch = "riscv64"))]
#[inline]
pub unsafe fn write_satp(_value: usize) {}

