use crate::global_constants::*;

use crate::trap::timer;

pub enum ProcessState {
    Start,
    Ready,
    Running,
    Waiting,
    Terminated,
}

pub const MAX_PROCESS_ID: usize = 256;
// Number of CPU registers
pub const NUM_CPU_REGISTERS: usize = 32;

pub struct ProcessControlBlock {
    // Current state of process i.e. running, waiting
    pub state: ProcessState,
    // Allow/disallow access to system resources
    pub privilege: u32,
    // Unique identification for each process
    pub pid: u32,
    // pid for the parent process
    pub parent_process: Option<u32>,
    // Time this process was created
    pub start_time: u32,
    // Time spent executing this process
    pub elapsed_time: u32,
    // Program Counter
    pub program_counter: u32,
    // CPU registers that process needs stored for execution in running state
    pub registers: [u32; NUM_CPU_REGISTERS],
}

impl ProcessControlBlock {
    // Creates a new init process, this should be the first process
    pub fn new_init() -> ProcessControlBlock {
        ProcessControlBlock { state: ProcessState::Start,
                              privilege: 0,
                              pid: 0,
                              parent_process: None,
                              registers: [0; NUM_CPU_REGISTERS],
                              start_time: 0, // TODO: actually write time
                              elapsed_time: 0,
                              program_counter: 0 }
    }

    // Creates a new process
    pub fn new(privileges: u32,
               id: u32,
               parent: Option<u32>,
               program_counter: u32)
               -> ProcessControlBlock {
        ProcessControlBlock { state: ProcessState::Start,
                              privilege: privileges,
                              pid: id,
                              parent_process: parent,
                              registers: [0; NUM_CPU_REGISTERS],
                              start_time: 0, // TODO: actually write time
                              elapsed_time: 0,
                              program_counter: program_counter }
    }
}
