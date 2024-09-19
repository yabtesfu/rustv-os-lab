#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Syscall {
    Write = 64,
    Exit = 93,
    Yield = 124,
}

impl TryFrom<usize> for Syscall {
    type Error = SyscallError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            64 => Ok(Self::Write),
            93 => Ok(Self::Exit),
            124 => Ok(Self::Yield),
            _ => Err(SyscallError::Unknown),
        }
    }
}

pub fn dispatch(number: usize, args: [usize; 6]) -> Result<usize, SyscallError> {
    match Syscall::try_from(number)? {
        Syscall::Write => Ok(args[2]),
        Syscall::Exit => Ok(0),
        Syscall::Yield => Ok(0),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyscallError {
    Unknown,
}

