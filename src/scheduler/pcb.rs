use crate::global_constants::*;

use crate::trap::timer;

#[derive(PartialEq)]
pub enum ProcessState {
    None,     // Process doesn't exist/is descheduled
    Running,  // Process is running (able to be switched to)
    Sleeping, // Process is sleeping
    Exited,   // Process is done, scheduler must remove it
}

pub const MAX_PROCESS_ID: usize = 256;
// Number of CPU registers
pub const NUM_CPU_REGISTERS: usize = 32;

pub struct ProcessControlBlock {
    // Current state of process i.e. running, waiting
    pub state: ProcessState,
    // Allow/disallow access to system resources
    pub privilege: usize,
    // Unique identification for each process
    pub pid: usize,
    // pid for the parent process
    pub parent_process: Option<usize>,
    // Time this process was created
    pub start_time: u64,
    // Time spent executing this process
    pub elapsed_time: u64,
    // Number of times this process has been context switched to
    pub switch_counter: usize,
    // PROCESS CONTEXT //
    // Program Counter
    pub program_counter: usize,
    // CPU registers that process needs stored for execution in running state
    pub registers: [u32; NUM_CPU_REGISTERS],
}

impl ProcessControlBlock {
    // Creates a new process
    pub fn new(privileges: usize,
               id: usize,
               parent: Option<usize>,
               program_counter: usize)
               -> ProcessControlBlock {
        ProcessControlBlock { state: ProcessState::Running,
                              privilege: privileges,
                              pid: id,
                              parent_process: parent,
                              registers: [0; NUM_CPU_REGISTERS],
                              start_time: 0, // TODO: actually write time
                              elapsed_time: 0,
                              switch_counter: 0,
                              program_counter: program_counter }
    }
}
