use crate::console::Console;
use crate::global_constants::MAX_PROC_COUNT;
use crate::{print, println};
use crate::utils::heapvec::HeapVec as HeapVec;
use core::fmt::Write;
use core::ptr::{read_volatile, write_volatile};
use pcb::{ProcessState as ProcessState, ProcessControlBlock as ProcessControlBlock};

pub mod pcb;

// The fixed amount of time each process runs before the
// round-robin scheduler switches to another process
const TIME_QUANTUM: usize = 10000;

extern "C" {
    static GLOBAL_CTX: *const u32;
}


pub struct Scheduler {
    current_index: usize,
    pid_counter: usize,
    processes: *mut HeapVec<ProcessControlBlock>,
}

impl Scheduler {
    // Creates a Scheduler with an init process
    pub fn new(p: *mut HeapVec<ProcessControlBlock>) -> Scheduler {
        Scheduler {
            current_index: 0,
            pid_counter: 0,
            processes: p,
        }
    }

    pub fn init(processes: *mut HeapVec<ProcessControlBlock>) -> Scheduler {
        let mut s = Scheduler::new(processes);
        let p_list: &mut HeapVec<ProcessControlBlock>;
        unsafe { p_list = s.processes.as_mut().unwrap(); }
        p_list.push(ProcessControlBlock::new(self.pid_counter));
        p_list[0].load_registers();
        s.pid_counter += 1;
        s
    }

    pub fn run(&mut self) {
        // Pick a process to switch to using the scheduling algorithm
        // Round Robin
        let p_list: &mut HeapVec<ProcessControlBlock>;
        unsafe { p_list = self.processes.as_mut().unwrap(); }
        let mut i = (self.current_index + 1) % p_list.size();
        while i != self.current_index {
            if p_list[i].state != ProcessState::Running {
                i = (i + 1) % p_list.size();
            }
        }
        let new_index = i;

        // Switch processes
        // Saving current process state and loading new process state
        self.processes[self.current_index].save_registers(mepc);
        let new_program_counter = self.processes[new_index].load_registers();
        self.processes[new_index].switch_counter += 1;

        // Gets the register context of the currently running process from
        // GLOBAL_CTX and stores it to the process at |self.current_index|
        p_list[self.current_index].load_registers();

        // Sets the new register context at GLOBAL_CTX to be the process at
        // |new_index| and then sets |self.current_index| to be equal to
        // |new_index|
        p_list[new_index].save_registers();

        self.current_index = new_index;
    }
}
