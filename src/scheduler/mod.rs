use crate::global_constants::*;
use crate::utils::heapvec;
use core::ptr::{read_volatile, write_volatile};

pub mod pcb;

// The fixed amount of time each process runs before the
// round-robin scheduler switches to another process
const TIME_QUANTUM: u32 = 10000;

pub struct Scheduler {
    processes: heapvec::HeapVec<pcb::ProcessControlBlock>,
}

impl Scheduler {
    // Creates a Scheduler with an init process
    pub fn new() -> Scheduler {
        let mut s = Scheduler { processes: heapvec::HeapVec::new(32) };
        // Create the init process as the first process
        s.processes.push(pcb::ProcessControlBlock::new_init());
        return s;
    }

    pub fn save_pcb(self, process_id: usize, mepc: u32) -> u32 {
        let p = self.processes[process_id];
        for i in 0..pcb::NUM_CPU_REGISTERS {
            p.registers[i] = read_volatile(GLOBAL_CTX + i);
        }
        p.state = pcb::ProcessState::Waiting;
        p.program_counter = mepc;
        return 1;
    }
}
