use core::fmt::Write;
use crate::console::Console;
use crate::global_constants::MAX_PROC_COUNT;
use crate::lock::Mutex;
use crate::{print, println};
use crate::utils::heapvec::HeapVec;
use pcb::{ProcessControlBlock, ProcessState};

pub mod pcb;

// The fixed amount of time each process runs before the
// round-robin scheduler switches to another process
const TIME_QUANTUM: u64 = 10000;

extern "C" {
    static GLOBAL_CTX: *const u32;
}

pub struct Scheduler {
    pub lock: Mutex,
    current_index: usize,
    pid_counter: usize,
    processes: *mut HeapVec<ProcessControlBlock>,

    // These allow us to delete processes in O(m) time where m is the number of
    // dead/exited processes instead of in O(n) time where n is the number of
    // total processes, up to |MAX_PROC_COUNT|. |dead_procs| holds the index of
    // a dead process at each index, with usize::MAX as the sentinal value.
    // |dead_index| holds the next index into |dead_procs| to store the next
    // dead process.
    dead_procs: [usize; MAX_PROC_COUNT],
    dead_index: usize,
}

impl Scheduler {
    // Creates a Scheduler with an init process
    fn new(p: *mut HeapVec<ProcessControlBlock>) -> Scheduler {
        Scheduler { current_index: 0,
                    pid_counter: 0,
                    lock: Mutex::new(),
                    dead_procs: [core::usize::MAX; MAX_PROC_COUNT],
                    dead_index: 0,
                    processes: p }
    }

    pub fn init(processes: *mut HeapVec<ProcessControlBlock>) -> Scheduler {
        let mut s = Scheduler::new(processes);
        let p_list: &mut HeapVec<ProcessControlBlock>;
        unsafe {
            p_list = s.processes.as_mut().unwrap();
        }

        // Create the "default" process, which is pid 0 for our operating
        // system, and will not hold a region of the heap for the stack, and is
        // thus the only process that is not dynamically allocated, the stack
        // pointer is already in place from boot time and program counter is
        // already executing at a particular address that we won't mess with
        p_list.push(ProcessControlBlock::default());
        p_list[0].set_pid(s.pid_counter);
        p_list[0].load_registers();
        s.pid_counter += 1;
        s
    }

    // Check the amount of time the current process has been running, if greater
    // than |TIME_QUANTUM|, swap to a new process, otherwise return
    pub fn run(&mut self) {
        // Pick a process to switch to using the scheduling algorithm
        // Round Robin
        let p_list: &mut HeapVec<ProcessControlBlock>;
        unsafe {
            p_list = self.processes.as_mut().unwrap();
        }

        let current_time = crate::trap::timer::get_current_time();

        // Check the running time of the current process against |TIME_QUANTUM|
        // so that each process gets some amount of time greater than just a few
        // clock ticks, if the currently running process hasn't had enough time,
        // just return without doing anything
        if current_time - p_list[self.current_index].start_time > TIME_QUANTUM {
            Scheduler::do_scheduler(self);
        }
        else {
            return;
        }
    }

    // Create a new process and add it to the process list where it will be run
    // periodically from the round robin scheduler
    pub fn create_proc(&mut self, func: fn() -> i32) -> Result<u32, ()> {
        let p_list: &mut HeapVec<ProcessControlBlock>;
        unsafe {
            p_list = self.processes.as_mut().unwrap();
        }

        // We currently don't support expanding the process vector
        if p_list.size() > MAX_PROC_COUNT {
            return Err(())
        }

        // We create a process by setting the memory address of the provided
        // function as the program counter for the new pcb
        let new_proc = ProcessControlBlock::init_new(self.pid_counter, func as u32);
        self.pid_counter += 1;

        // Add the new process to the list of processes
        p_list.push(new_proc);

        // Return the pid
        Ok(self.pid_counter as u32 - 1)
    }

    // Deallocate processes that have exited that are still in the process list
    // TODO: Actually deallocate these
    pub fn delete_proc(&mut self) {
        let p_list: &mut HeapVec<ProcessControlBlock>;
        unsafe {
            p_list = self.processes.as_mut().unwrap();
        }

        for i in 0..self.dead_index {
            if self.dead_procs[i] != core::usize::MAX {
                p_list[self.dead_procs[i]].state = ProcessState::None;
            }
            else {
                break;
            }
            self.dead_procs[i] = core::usize::MAX;
        }

        self.dead_index = 0;
    }

    // Print a nice table of PIDs with states
    // TODO: Add other things to print, like names, total running time, priority, etc.
    pub fn print(&self) {
        let p_list: &mut HeapVec<ProcessControlBlock>;
        unsafe {
            p_list = self.processes.as_mut().unwrap();
        }

        println!("current pid: {}", self.current_index);

        println!("{PID:>width$} {STATE:>width$}", PID="PID", STATE="STATE", width=15);
        for p in p_list.iter() {
            println!("{pid:>width$} {state:>width$}", pid=p.pid, state=p.state, width=15);
        }
    }

    // Actually perform the round robin schedule to swap in a new process.
    fn do_scheduler(scheduler: &mut Scheduler) {
        let p_list: &mut HeapVec<ProcessControlBlock>;
        unsafe {
            p_list = scheduler.processes.as_mut().unwrap();
        }

        let mut i = (scheduler.current_index + 1) % p_list.size();
        while i != scheduler.current_index && p_list[i].state != ProcessState::Running {
            if p_list[i].state == ProcessState::Exited {
                scheduler.dead_procs[scheduler.dead_index] = i;
                scheduler.dead_index += 1;
            }
            i = (i + 1) % p_list.size();
        }
        let new_index = i;

        // Gets the register context of the currently running process from
        // GLOBAL_CTX and stores it to the process at |scheduler.current_index|
        p_list[scheduler.current_index].load_registers();

        // Sets the new register context at GLOBAL_CTX to be the process at
        // |new_index| and then sets |scheduler.current_index| to be equal to
        // |new_index|
        p_list[new_index].set_global_ctx();
        p_list[new_index].start_time = crate::trap::timer::get_current_time();

        scheduler.current_index = new_index;
    }
}
