use crate::global_constants::*;

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
    pub privileges: u32,
    // Unique identification for each process
    pub pid: u32,
    // pid for the parent process
    pub parent_process: Option<u32>,
    // Time this process was created
    pub start_time: u32,
    // Time spent executing this process
    pub elapsed_time: u32,
    //Program Counter
    pub program_counter: u32,
    // CPU registers that process needs stored for execution in running state
    pub registers: [u32; NUM_CPU_REGISTERS],
}

impl ProcessControlBlock {
    pub fn new_init() -> ProcessControlBlock {
        ProcessControlBlock { state: ProcessState::Start,
                              privileges: 0,
                              pid: 0,
                              parent_process: None,
                              registers: [0; NUM_CPU_REGISTERS],
                              start_time: 0,
                              elapsed_time: 0,
                              program_counter: 0}
    }
}
