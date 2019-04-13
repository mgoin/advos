use core::fmt::Write;
use crate::console::Console;
use crate::global_constants::MAX_PROC_COUNT;
use crate::{print, println};
use crate::utils::heapvec::HeapVec;
use pcb::{ProcessControlBlock, ProcessState};

pub mod pcb;

// The fixed amount of time each process runs before the
// round-robin scheduler switches to another process
const TIME_QUANTUM: u64 = 10000;

pub extern "C" fn recover() {
    println!("in recover");
    //TODO add ecall here to call exit() system call
}

pub struct Scheduler {
    current_index: usize,
    pid_counter: usize,
    processes: *mut HeapVec<ProcessControlBlock>,
}

impl Scheduler {
    // Creates a Scheduler with an init process
    pub fn new() -> Scheduler {
        Scheduler { current_index: 0,
                    pid_counter: 0,
                    processes: core::ptr::null_mut(), }
    }

    pub fn init(processes: *mut HeapVec<ProcessControlBlock>)
                -> *mut Scheduler {
        let s: *mut Scheduler = crate::memman::MemManager::kmalloc(core::mem::size_of::<Scheduler>()).unwrap() as *mut Scheduler;
        unsafe {
            core::ptr::write_volatile(s, Scheduler::new());
        }

        // Create the "default" process, which is pid 0 for our operating
        // system, and will not hold a region of the heap for the stack, and is
        // thus the only process that is not dynamically allocated, the stack
        // pointer is already in place from boot time and program counter is
        // already executing at a particular address that we won't mess with
        unsafe {
            (*processes).push(ProcessControlBlock::default());
            (*processes)[0].set_pid((*s).pid_counter);
            (*s).pid_counter += 1;
            (*s).processes = processes;
        };
        s
    }

    // Check the amount of time the current process has been running, if greater
    // than |TIME_QUANTUM|, swap to a new process, otherwise return
    pub fn run(&mut self, mepc: u32) -> u32 {
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
            Scheduler::do_scheduler(self, mepc)
        }
        else {
            mepc
        }
    }

    // Create a new process and add it to the process list where it will be run
    // periodically from the round robin scheduler
    pub fn create_proc(&mut self, func: fn() -> i32) -> Result<u32, ()> {
        let p_list: &mut HeapVec<ProcessControlBlock>;
        unsafe {
            p_list = self.processes.as_mut().unwrap();
        }

        // We create a process by setting the memory address of the provided
        // function as the program counter for the new pcb

        // We currently don't support expanding the process vector
        if p_list.size() > MAX_PROC_COUNT {
            return Err(())
        }

        // If there weren't any spots taken by |ProcessState::Exited| processes,
        // push a new process onto the vector.
        p_list.push(ProcessControlBlock::init_new(self.pid_counter, func as u32, recover as u32));

        let pid = self.pid_counter as u32;
        self.pid_counter += 1;

        // Return the pid
        Ok(pid)
    }

    // Deallocate processes that have exited that are still in the process list
    // TODO: Actually deallocate these
    pub fn delete_proc(&mut self, pid: u32) {
        let p_list: &mut HeapVec<ProcessControlBlock>;
        unsafe {
            p_list = self.processes.as_mut().unwrap();
        }

        let ind: usize = if pid == 0 {
            self.current_index
        }
        else {
            pid as usize
        };

        p_list[ind].state = ProcessState::Exited;

        // If the process is at the end of the process list, we can remove it in
        // constant time, so we'll simply deallocate it here. Otherwise, we'll
        // just mark it as exited and overwrite when a new process is created.
        if ind == p_list.size() {
            core::mem::drop(p_list.pop().unwrap());
        }
    }

    pub fn get_current_proc(&mut self) -> &mut ProcessControlBlock {
        let p_list: &mut HeapVec<ProcessControlBlock>;
        unsafe { p_list = self.processes.as_mut().unwrap(); }
        &mut p_list[self.current_index]
    }

    // Print a nice table of PIDs with states
    // TODO: Add other things to print, like names, total running time, priority, etc.
    pub fn print(&mut self) {
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
    fn do_scheduler(scheduler: &mut Scheduler, mepc: u32) -> u32 {
        let p_list: &mut HeapVec<ProcessControlBlock>;
        unsafe {
            p_list = scheduler.processes.as_mut().unwrap();
        }

        let mut i = (scheduler.current_index + 1) % p_list.size();
        while i != scheduler.current_index && p_list[i].state != ProcessState::Running {
            i = (i + 1) % p_list.size();
        }
        let new_index = i;

        // Gets the register context of the currently running process from
        // GLOBAL_CTX and stores it to the process at |scheduler.current_index|
        p_list[scheduler.current_index].load_registers(mepc);

        // Sets the new register context at GLOBAL_CTX to be the process at
        // |new_index| and then sets |scheduler.current_index| to be equal to
        // |new_index|
        let new_pc = p_list[new_index].set_global_ctx();
        p_list[new_index].start_time = crate::trap::timer::get_current_time();

        scheduler.current_index = new_index;
        new_pc
    }
}
