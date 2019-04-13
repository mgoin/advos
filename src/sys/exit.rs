use crate::scheduler::pcb::{ProcessControlBlock, ProcessState};
use crate::GLOBAL_SCHED;

use super::ecall::ecall;
use super::table::SyscallTable;

pub fn exit(status: u32) {
    ecall(SyscallTable::EXIT, status);
}

pub fn _exit(status: u32) {
    let p: &mut ProcessControlBlock;
    unsafe {
        p = (*GLOBAL_SCHED).get_current_proc();
    }

    p.state = ProcessState::Exited;
}
