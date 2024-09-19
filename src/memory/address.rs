pub const KERNEL_BASE: usize = 0x8000_0000;
pub const PAGE_SIZE: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryRegion {
    pub start: usize,
    pub end: usize,
}

impl MemoryRegion {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub const fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub const fn contains(&self, address: usize) -> bool {
        address >= self.start && address < self.end
    }

    pub const fn is_page_aligned(&self) -> bool {
        self.start % PAGE_SIZE == 0 && self.end % PAGE_SIZE == 0
    }
}

pub const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

pub const fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

