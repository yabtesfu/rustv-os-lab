pub mod csr;

#[inline]
pub fn wfi() {
    #[cfg(target_arch = "riscv64")]
    unsafe {
        core::arch::asm!("wfi", options(nomem, nostack, preserves_flags));
    }

    #[cfg(not(target_arch = "riscv64"))]
    core::hint::spin_loop();
}

