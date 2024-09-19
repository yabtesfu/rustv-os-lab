#[inline]
pub unsafe fn read8(base: usize, offset: usize) -> u8 {
    ((base + offset) as *const u8).read_volatile()
}

#[inline]
pub unsafe fn write8(base: usize, offset: usize, value: u8) {
    ((base + offset) as *mut u8).write_volatile(value);
}

#[inline]
pub unsafe fn read32(base: usize, offset: usize) -> u32 {
    ((base + offset) as *const u32).read_volatile()
}

#[inline]
pub unsafe fn write32(base: usize, offset: usize, value: u32) {
    ((base + offset) as *mut u32).write_volatile(value);
}

