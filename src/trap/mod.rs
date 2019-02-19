use crate::{print, println};
use crate::console::Console;
use crate::global_constants::CORE_LOCAL_INTERRUPT_MAP;
use core::fmt::Write;
use core::ptr::{write_volatile, read_volatile};

pub mod timer;

#[no_mangle]
pub extern "C" fn handle_trap(mcause: u32, mepc: u32) -> u32 {
  println!("0x{:x}", mcause);

  if (mcause >> 31) == 1 {
    // TODO: HandleInterrupt(mcause);
    println!("got an interrupt");
    match mcause {
      0x80000007 => {
        timer::incr_timer().unwrap();
        return mepc;
      },
      _ => (),
    }
  }
  else {
    // TODO: HandleException(mcause);
    println!("got an exception");
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
