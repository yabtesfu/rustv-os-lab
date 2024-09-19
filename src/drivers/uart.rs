use core::fmt;

use super::mmio;

pub const UART0_BASE: usize = 0x1000_0000;

const RBR: usize = 0;
const THR: usize = 0;
const IER: usize = 1;
const FCR: usize = 2;
const LCR: usize = 3;
const LSR: usize = 5;

const LSR_DATA_READY: u8 = 1 << 0;
const LSR_THR_EMPTY: u8 = 1 << 5;

pub struct Uart {
    base: usize,
}

impl Uart {
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    pub fn init(&self) {
        unsafe {
            mmio::write8(self.base, IER, 0x00);
            mmio::write8(self.base, LCR, 0x03);
            mmio::write8(self.base, FCR, 0x01);
        }
    }

    pub fn put(&self, byte: u8) {
        unsafe {
            while mmio::read8(self.base, LSR) & LSR_THR_EMPTY == 0 {}
            mmio::write8(self.base, THR, byte);
        }
    }

    pub fn try_get(&self) -> Option<u8> {
        unsafe {
            if mmio::read8(self.base, LSR) & LSR_DATA_READY != 0 {
                Some(mmio::read8(self.base, RBR))
            } else {
                None
            }
        }
    }

    pub fn write_bytes(&self, bytes: &[u8]) {
        for &byte in bytes {
            if byte == b'\n' {
                self.put(b'\r');
            }
            self.put(byte);
        }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}

