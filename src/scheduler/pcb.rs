use crate::global_constants::*;
use core::ptr::{read_volatile, write_volatile};

use crate::trap::timer;

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
    pub program_counter: usize,
    // CPU registers that process needs stored for execution in running state
    pub registers: [u32; NUM_CPU_REGISTERS],
}

impl ProcessControlBlock {
    // Creates a new process
    pub fn new(id: usize) -> ProcessControlBlock {
        ProcessControlBlock { state: ProcessState::Running,
                              pid: id,
                              registers: [0; NUM_CPU_REGISTERS],
                              program_counter: 0 }
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
    pub fn save_registers(&mut self) {
        for i in 0..NUM_CPU_REGISTERS {
            unsafe {
                write_volatile(GLOBAL_CTX.add(i), self.registers[i]);
            }
        }

        unsafe {
            asm!("csrw mepc, $0" : "=r"(self.program_counter) ::: "volatile");
        }
    }
}
