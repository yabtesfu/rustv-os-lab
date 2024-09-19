#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrapCause {
    InstructionMisaligned,
    IllegalInstruction,
    Breakpoint,
    EnvironmentCallFromUser,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    SupervisorTimerInterrupt,
    Unknown { interrupt: bool, code: usize },
}

impl TrapCause {
    pub fn from_scause(scause: usize) -> Self {
        let interrupt = scause >> (usize::BITS - 1) == 1;
        let code = scause & !(1usize << (usize::BITS - 1));

        match (interrupt, code) {
            (false, 0) => Self::InstructionMisaligned,
            (false, 2) => Self::IllegalInstruction,
            (false, 3) => Self::Breakpoint,
            (false, 8) => Self::EnvironmentCallFromUser,
            (false, 12) => Self::InstructionPageFault,
            (false, 13) => Self::LoadPageFault,
            (false, 15) => Self::StorePageFault,
            (true, 5) => Self::SupervisorTimerInterrupt,
            _ => Self::Unknown { interrupt, code },
        }
    }
}

