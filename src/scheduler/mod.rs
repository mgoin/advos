use crate::console::Console;
use crate::global_constants::*;
use crate::utils::heapvec::HeapVec as HeapVec;
use core::fmt::Write;
use core::ptr::{read_volatile, write_volatile};
use pcb::ProcessState as ProcessState;
use pcb::ProcessControlBlock as ProcessControlBlock;

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
    processes: HeapVec<ProcessControlBlock>,
}

impl Scheduler {
    // Creates a Scheduler with an init process
    pub fn new() -> Scheduler {
        Scheduler {
            current_index: 0,
            pid_counter: 0,
            processes: HeapVec::new(16),
        }
    }

    pub fn init(&mut self) {
        let s = Scheduler::new();
        self.processes.push(ProcessControlBlock::new(self.pid_counter));
        self.load_proc();
        self.pid_counter += 1;
    }

    pub fn run(&mut self) {
        // Pick a process to switch to using the scheduling algorithm
        // Round Robin
        crate::print!("running scheduler\n");
        let mut i = (self.current_index + 1) % self.processes.size();
        while i != self.current_index {
            if self.processes[i].state != ProcessState::Running {
                i = (i + 1) % self.processes.size();
            }
        }
        let new_index = i;

        // Switch registers 

        // Gets the register context of the currently running process from
        // GLOBAL_CTX and stores it to the process at |self.current_index|
        self.load_proc();

        // Sets the new register context at GLOBAL_CTX to be the process at
        // |new_index| and then sets |self.current_index| to be equal to
        // |new_index|
        self.schedule_new_proc(new_index);

        // Done??
    }

    fn load_proc(&mut self) {
        let mut offset: usize = 0;
        unsafe {
            for mut i in self.processes[self.current_index].registers.iter_mut() {
                write_volatile(&mut i, &mut (*(GLOBAL_CTX as *mut u32).add(offset)));
                offset += 1;
            }

            asm!("csrr $0, mepc" :: "r"(self.processes[self.current_index].program_counter) :: "volatile");
        }
    }

    fn schedule_new_proc(&mut self, index: usize) {
        unsafe {
            let mut offset: usize = 0;
            for i in self.processes[index].registers.iter_mut() {
                write_volatile(&mut (*(GLOBAL_CTX as *mut u32).add(offset)), *i);
                offset += 1;
            }

            asm!("csrw  mepc, $0" :: "r"(self.processes[index].program_counter) :: "volatile");
        }

        self.current_index = index;
    }

    pub fn save_pcb(mut self, process_id: usize, mepc: usize) -> usize {
        let p = &mut self.processes[process_id];
        for i in 0..pcb::NUM_CPU_REGISTERS {
            unsafe { p.registers[i] = read_volatile(GLOBAL_CTX.add(i)); }
        }
        p.state = ProcessState::Running;
        p.program_counter = mepc;
        return 1;
    }
}
