pub mod cause;
pub mod frame;

use cause::TrapCause;
use frame::TrapFrame;

pub fn handle_trap(frame: &mut TrapFrame, scause: usize, stval: usize) -> TrapCause {
    let cause = TrapCause::from_scause(scause);
    match cause {
        TrapCause::SupervisorTimerInterrupt => {
            frame.sepc = frame.sepc.wrapping_add(4);
        }
        TrapCause::EnvironmentCallFromUser => {
            frame.sepc = frame.sepc.wrapping_add(4);
        }
        TrapCause::LoadPageFault | TrapCause::StorePageFault | TrapCause::InstructionPageFault => {
            frame.bad_address = stval;
        }
        _ => {}
    }
    cause
}

