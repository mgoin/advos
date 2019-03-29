use crate::utils::heapvec;
use core::ptr::{read_volatile, write_volatile};

pub mod pcb;

pub struct Scheduler {
    temp: pcb::ProcessControlBlock,
}

impl Scheduler {
    pub fn save_pcb(self, mepc: u32) -> u32 {
        self.temp = pcb::ProcessControlBlock::new_init();
        for i in 0..pcb::NUM_CPU_REGISTERS {
            self.temp.registers[i] = read_volatile(GLOBAL_CTX + i);
        }
        self.temp.state = pcb::ProcessState::Waiting;
        self.temp.program_counter = mepc;
        return 1;
    }
}
