use crate::global_constants::*;
use crate::utils::heapvec;

enum ProcessState {
    Start,
    Ready,
    Running,
    Waiting,
    Terminated,
}

const MAX_PROCESS_ID: usize = 256;
// Number of CPU registers
const NUM_CPU_REGISTERS: usize = 32;

pub struct ProcessControlBlock {
    // Current state of process i.e. running, waiting
    state: ProcessState,
    // Allow/disallow access to system resources
    privileges: u32,
    // Unique identification for each process
    pid: u32,
    // pid for the parent process
    parent_process: Option<u32>,
    // CPU registers that process needs stored for execution in running state
    registers: [u8; NUM_CPU_REGISTERS],
    // Time this process was created
    start_time: u32,
    // Time spent executing this process
    elapsed_time: u32,
}

impl ProcessControlBlock {
    pub fn new_init() -> ProcessControlBlock {
        ProcessControlBlock { state: ProcessState::Start,
                              privileges: 0,
                              pid: 0,
                              parent_process: None,
                              registers: [0; NUM_CPU_REGISTERS],
                              start_time: 0,
                              elapsed_time: 0 }
    }
}
