use crate::{print, println};
use crate::console::Console;
use crate::global_constants::CORE_LOCAL_INTERRUPT_MAP;
use core::fmt::Write;
use core::ptr::{write_volatile, read_volatile};

pub mod timer;

#[no_mangle]
pub extern "C" fn handle_trap(mcause: u32, mepc: u32) -> u32 {
  let interrupt_flag: u32 = mcause >> 31;
  let exception_code: u32 = mcause & 0x1F;

  if interrupt_flag == 1 {
    // TODO: HandleInterrupt(mcause);
    println!("Got an interrupt");
  }
  else {
    // TODO: HandleException(mcause);
    println!("Got an exception");
  }

  println!("Code: 0x{:x}", exception_code);

  // Match the flag and code to see what happened
  match (interrupt_flag, exception_code) {
    (1, 0)  => println!("User software interrupt"),
    (1, 1)  => println!("Supervisor software interrupt"),
    (1, 3)  => println!("Machine software interrupt"),
    (1, 4)  => println!("User timer interrupt"),
    (1, 5)  => println!("Supervisor timer interrupt"),
    (1, 7)  => {
      println!("Machine timer interrupt");
      timer::incr_timer().unwrap();
    },
    (1, 8)  => println!("User external interrupt"),
    (1, 9)  => println!("Supervisor external interrupt"),
    (1, 11) => println!("Machine external interrupt"),
    (0, 0)  => println!("Instruction address misaligned"),
    (0, 1)  => println!("Instruction access fault"),
    (0, 2)  => println!("Illegal instruction"),
    (0, 3)  => println!("Breakpoint"),
    (0, 4)  => println!("Load address misaligned"),
    (0, 5)  => println!("Load access fault"),
    (0, 6)  => println!("Store/AMO address misaligned"),
    (0, 7)  => println!("Store/AMO access fault"),
    (0, 8)  => println!("Environment call from U-mode"),
    (0, 9)  => println!("Environment call from S-mode"),
    (0, 11) => println!("Environment call from M-mode"),
    (0, 12) => println!("Instruction page fault"),
    (0, 13) => println!("Load page fault"),
    (0, 15) => println!("Store/AMO page fault"),
    (_, _)  => println!("Reserved/unknown code (THIS SHOULD NEVER HAPPEN)"),
  }

  // Clear the CLIM to indicate we've handled the interrupt
  let clim = CORE_LOCAL_INTERRUPT_MAP as *mut u32;
  unsafe { write_volatile(clim, 0); }
  println!("trap handled, returning");

  let mepc_ptr = mepc as *mut u32;
  let next_instruction: u32;
  unsafe {
    next_instruction = read_volatile(mepc_ptr);
  }

  // Compressed instructions are 2 bytes, while uncompressed are 4 bytes.
  // If the lowest 2 bits of the instruction are 0b00, then the instruction is
  // uncompressed, and if anything else, then the instruction is compressed, so
  // we can then determine how much to increment mepc by to return to the
  // correct instruction after the trap has been handled.
  if (next_instruction & 0x3) != 0 {
    mepc + 2
  }
  else {
    mepc + 4
  }
}
