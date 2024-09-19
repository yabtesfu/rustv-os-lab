#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrapFrame {
    pub registers: [usize; 32],
    pub sepc: usize,
    pub sstatus: usize,
    pub bad_address: usize,
}

impl TrapFrame {
    pub const fn zeroed() -> Self {
        Self {
            registers: [0; 32],
            sepc: 0,
            sstatus: 0,
            bad_address: 0,
        }
    }
}

