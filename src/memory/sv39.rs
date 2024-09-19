pub const PAGE_SIZE: usize = 4096;
pub const ENTRIES_PER_TABLE: usize = 512;
pub const SATP_MODE_SV39: usize = 8 << 60;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sv39Address(usize);

impl Sv39Address {
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    pub const fn offset(self) -> usize {
        self.0 & 0xfff
    }

    pub const fn vpn(self) -> [usize; 3] {
        [
            (self.0 >> 12) & 0x1ff,
            (self.0 >> 21) & 0x1ff,
            (self.0 >> 30) & 0x1ff,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PteFlags(u64);

impl PteFlags {
    pub const VALID: Self = Self(1 << 0);
    pub const READ: Self = Self(1 << 1);
    pub const WRITE: Self = Self(1 << 2);
    pub const EXECUTE: Self = Self(1 << 3);
    pub const USER: Self = Self(1 << 4);
    pub const GLOBAL: Self = Self(1 << 5);
    pub const ACCESSED: Self = Self(1 << 6);
    pub const DIRTY: Self = Self(1 << 7);

    pub const fn bits(self) -> u64 {
        self.0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn new(ppn: usize, flags: PteFlags) -> Self {
        Self(((ppn as u64) << 10) | flags.bits())
    }

    pub const fn bits(self) -> u64 {
        self.0
    }

    pub const fn ppn(self) -> usize {
        (self.0 >> 10) as usize
    }

    pub const fn flags(self) -> PteFlags {
        PteFlags(self.0 & 0x3ff)
    }

    pub const fn is_valid(self) -> bool {
        self.flags().contains(PteFlags::VALID)
    }

    pub const fn is_leaf(self) -> bool {
        let flags = self.flags();
        flags.contains(PteFlags::READ) ||
            flags.contains(PteFlags::WRITE) ||
            flags.contains(PteFlags::EXECUTE)
    }
}

