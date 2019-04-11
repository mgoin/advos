use crate::global_constants::{NUM_CPU_REGISTERS, PROC_ALLOC_SIZE};
use crate::memman::MemManager;

extern "C" {
    static mut GLOBAL_CTX: [u32;32];
}

// TODO: Probably need to add some more states here for better granularity
#[derive(PartialEq)]
pub enum ProcessState {
    None,     // Process doesn't exist/is descheduled
    Running,  // Process is running (able to be switched to)
    Sleeping, // Process is sleeping
    Exited,   // Process is done, scheduler must remove it
}

// Allow us to do some formatted printing of the ProcessState in
// Scheduler::Print()
impl core::fmt::Display for ProcessState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let width = f.width().unwrap();
        match self {
            ProcessState::None => write!(f, "{:>w$}", "None", w=width),
            ProcessState::Running => write!(f, "{:>w$}", "Running", w=width),
            ProcessState::Sleeping => write!(f, "{:>w$}", "Sleeping", w=width),
            ProcessState::Exited => write!(f, "{:>w$}", "Exited", w=width),
        }
    }
}

const RETURN_ADDRESS_REGISTER_OFFSET: usize = 1;
const STACK_POINTER_REGISTER_OFFSET: usize = 2;

pub struct ProcessControlBlock {
    // Current state of process i.e. running, waiting
    pub state: ProcessState,
    // Unique identification for each process
    pub pid: usize,

    pub start_time: u64,
    // PROCESS CONTEXT //
    // Program Counter
    program_counter: u32,
    // CPU registers that process needs stored for execution in running state
    registers: [u32; NUM_CPU_REGISTERS],

    start_fn: u32,
    end_fn: u32,

    // We'll have to allocate a region of memory for the stack.
    // |stack_start| will point to the bottom of the region and |stack_end| will
    // point to the top, i.e. |stack_start| = |stack_end| + PROC_ALLOC_SIZE
    stack_end: *const u32,
    stack_start: *mut u32,
}

impl ProcessControlBlock {
    // Creates a new process
    fn new(id: usize, start_func: u32, end_func: u32) -> ProcessControlBlock {
        ProcessControlBlock { state: ProcessState::Running,
                              pid: id,
                              start_time: 0,
                              registers: [0; NUM_CPU_REGISTERS],
                              program_counter: start_func,
                              start_fn: start_func,
                              end_fn: end_func,
                              stack_end: MemManager::kmalloc(PROC_ALLOC_SIZE).unwrap() as *const u32,
                              stack_start: core::ptr::null_mut(),
        }
    }

    pub fn init_new(pid: usize, start_func: u32, end_func: u32) -> ProcessControlBlock {
        let mut pcb = ProcessControlBlock::new(pid, start_func, end_func);
        unsafe {
            // Set the stack pointer to be the bottom of the allocated stack
            // region
            pcb.stack_start = pcb.stack_end.add(PROC_ALLOC_SIZE) as *mut u32;
            pcb.registers[RETURN_ADDRESS_REGISTER_OFFSET] = pcb.end_fn;
            pcb.registers[STACK_POINTER_REGISTER_OFFSET] = pcb.stack_start as u32;
        }

        pcb
    }

    // Loads the cpu registers so another process can run
    pub fn load_registers(&mut self, mepc: u32) {
        for i in 0..NUM_CPU_REGISTERS {
            unsafe {
                self.registers = GLOBAL_CTX;
            }
        }
        self.program_counter = mepc;
    }

    // Saves the process registers onto the cpu so it can run
    pub fn set_global_ctx(&mut self) -> u32 {
        for i in 0..NUM_CPU_REGISTERS {
            unsafe {
                GLOBAL_CTX = self.registers;
            }
        }

        unsafe {
            asm!("csrw mepc, $0" : "=r"(&mut self.program_counter) ::: "volatile");
        }
        self.program_counter
    }

    pub fn set_pid(&mut self, pid: usize) {
        self.pid = pid;
    }
}

impl Drop for ProcessControlBlock {
    fn drop(&mut self) {
        if !self.stack_end.is_null() {
          MemManager::kfree(self.stack_end as u32).unwrap();
        }
    }
}

// Do not allocate anything or set any meaningful state, this is primarily used
// for the initial Scheduler process during initialization.
impl Default for ProcessControlBlock {
    fn default() -> Self {
        ProcessControlBlock {
            state: ProcessState::Running,
            pid: 0,
            start_time: 0,
            registers: [0; NUM_CPU_REGISTERS],
            program_counter: 0,
            start_fn: 0,
            end_fn: crate::scheduler::recover as u32,
            stack_end: core::ptr::null(),
            stack_start: core::ptr::null_mut(),
        }
    }
}
