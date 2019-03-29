use crate::global_constants::*;
use crate::utils::heapvec;
use core::ptr::{read_volatile, write_volatile};

pub mod pcb;

// The fixed amount of time each process runs before the
// round-robin scheduler switches to another process
const TIME_QUANTUM: usize = 10000;

extern "C" {
    static GLOBAL_CTX: *const u32;
}

pub struct Scheduler {
    current_index: usize,
    processes: heapvec::HeapVec<pcb::ProcessControlBlock>,
}

impl Scheduler {
    // Creates a Scheduler with an init process
    pub fn new() -> Scheduler {
        let mut s = Scheduler { current_index: 0,
                                processes: heapvec::HeapVec::new(32) };
        // Create the init process as the first process
        s.processes.push(pcb::ProcessControlBlock::new(0, 0, None, 0)).unwrap();
        return s;
    }

    pub fn context_switch(self) {
        // Pick a process to switch to using the scheduling algorithm
        // Round Robin
        let mut i = self.current_index + 1;
        while i != self.current_index {
            if i == self.processes.size() - 1 {
                i = 0;
            }
            else if self.processes[i].state != pcb::ProcessState::Running {
                i += 1;
            }
            else {
                break;
            }
        }
        let new_index = i;

        // Switch registers 

        // Done??
    }

    pub fn save_pcb(mut self, process_id: usize, mepc: usize) -> usize {
        let p = &mut self.processes[process_id];
        for i in 0..pcb::NUM_CPU_REGISTERS {
            unsafe { p.registers[i] = read_volatile(GLOBAL_CTX.add(i)); }
        }
        p.state = pcb::ProcessState::Running;
        p.program_counter = mepc;
        return 1;
    }
}
