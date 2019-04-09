use crate::global_constants::{NUM_CPU_REGISTERS, PROC_ALLOC_SIZE};
use crate::memman::MemManager;
use core::ptr::{read_volatile, write_volatile};

extern "C" {
    static GLOBAL_CTX: *mut u32;
}

#[derive(PartialEq)]
pub enum ProcessState {
    None,     // Process doesn't exist/is descheduled
    Running,  // Process is running (able to be switched to)
    Sleeping, // Process is sleeping
    Exited,   // Process is done, scheduler must remove it
}

pub const MAX_PROCESS_ID: usize = 32;
// Number of CPU registers
pub const NUM_CPU_REGISTERS: usize = 32;

pub struct ProcessControlBlock {
    // Current state of process i.e. running, waiting
    pub state: ProcessState,
    // Unique identification for each process
    pub pid: usize,
    // PROCESS CONTEXT //
    // Program Counter
    program_counter: u32,
    // CPU registers that process needs stored for execution in running state
    registers: [u32; NUM_CPU_REGISTERS],

    // We'll have to allocate a region of memory for the stack.
    // |stack_start| will point to the bottom of the region and |stack_end| will
    // point to the top, i.e. |stack_start| = |stack_end| + PROC_ALLOC_SIZE
    stack_end: *const u32,
    stack_start: *mut u32,
}

impl ProcessControlBlock {
    // Creates a new process
    fn new(id: usize, pc: u32) -> ProcessControlBlock {
        ProcessControlBlock { state: ProcessState::Running,
                              pid: id,
                              registers: [0; NUM_CPU_REGISTERS],
                              program_counter: pc,
                              stack_end: MemManager::kmalloc(PROC_ALLOC_SIZE).unwrap() as *const u32,
                              stack_start: core::ptr::null_mut(),
        }
    }

    pub fn init_new(pid: usize, pc: u32) -> ProcessControlBlock {
        let mut pcb = ProcessControlBlock::new(pid, pc);
        unsafe {
            // Set the stack pointer to be the bottom of the allocated stack
            // region
            pcb.stack_start = pcb.stack_end.add(PROC_ALLOC_SIZE) as *mut u32;
            pcb.registers[STACK_POINTER_REGISTER_OFFSET] = pcb.stack_start as u32;
        }

        pcb
    }

    // Loads the cpu registers so another process can run
    pub fn load_registers(&mut self) {
        for i in 0..NUM_CPU_REGISTERS {
            unsafe {
                self.registers[i] = read_volatile(GLOBAL_CTX.add(i));
            }
        }
        unsafe {
            asm!("csrr $0, mepc" :: "r"(self.program_counter) :: "volatile")
        };
    }

    // Saves the process registers onto the cpu so it can run
    pub fn set_global_ctx(&mut self) {
        for i in 0..NUM_CPU_REGISTERS {
            unsafe {
                write_volatile(GLOBAL_CTX.add(i), self.registers[i]);
            }
        }

        unsafe {
            asm!("csrw mepc, $0" : "=r"(self.program_counter) ::: "volatile");
        }
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
            stack_end: core::ptr::null(),
            stack_start: core::ptr::null_mut(),
        }
    }
}
