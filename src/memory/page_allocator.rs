pub const PAGE_SIZE: usize = 4096;

const TAKEN: u8 = 1 << 0;
const LAST: u8 = 1 << 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageDescriptor {
    flags: u8,
}

impl PageDescriptor {
    pub const fn empty() -> Self {
        Self { flags: 0 }
    }

    pub const fn is_free(&self) -> bool {
        self.flags & TAKEN == 0
    }

    pub const fn is_last(&self) -> bool {
        self.flags & LAST != 0
    }

    fn mark_taken(&mut self, last: bool) {
        self.flags = TAKEN | if last { LAST } else { 0 };
    }

    fn clear(&mut self) {
        self.flags = 0;
    }
}

pub struct PageAllocator<'a> {
    heap_start: usize,
    pages: &'a mut [PageDescriptor],
}

impl<'a> PageAllocator<'a> {
    pub fn new(heap_start: usize, pages: &'a mut [PageDescriptor]) -> Self {
        Self { heap_start, pages }
    }

    pub fn alloc_contiguous(&mut self, count: usize) -> Option<usize> {
        if count == 0 || count > self.pages.len() {
            return None;
        }

        for start in 0..=self.pages.len() - count {
            let free = self.pages[start..start + count]
                .iter()
                .all(PageDescriptor::is_free);

            if free {
                for offset in 0..count {
                    self.pages[start + offset].mark_taken(offset == count - 1);
                }
                return Some(self.heap_start + start * PAGE_SIZE);
            }
        }

        None
    }

    pub fn dealloc(&mut self, address: usize) -> Result<(), AllocError> {
        if address < self.heap_start || (address - self.heap_start) % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidAddress);
        }

        let mut index = (address - self.heap_start) / PAGE_SIZE;
        if index >= self.pages.len() || self.pages[index].is_free() {
            return Err(AllocError::InvalidAddress);
        }

        loop {
            let last = self.pages[index].is_last();
            self.pages[index].clear();
            if last {
                return Ok(());
            }
            index += 1;
            if index >= self.pages.len() {
                return Err(AllocError::CorruptRun);
            }
        }
    }

    pub fn free_pages(&self) -> usize {
        self.pages.iter().filter(|page| page.is_free()).count()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AllocError {
    InvalidAddress,
    CorruptRun,
}

