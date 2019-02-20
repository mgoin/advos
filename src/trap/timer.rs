use crate::{println, print};
use crate::global_constants::{CLOCK_FREQ, CORE_LOCAL_INTERRUPT_MAP};
use crate::console::Console;
use core::fmt::{Write,Error};
use core::ptr::{write_volatile, read_volatile};

const CTX_PER_SECOND    : u64 = 1;
const TIME_TO_CTX_SWITCH: u64 = CLOCK_FREQ / CTX_PER_SECOND;

const MTIME_CMP_LO: u64 = CORE_LOCAL_INTERRUPT_MAP + 0x4000;
const MTIME_CMP_HI: u64 = CORE_LOCAL_INTERRUPT_MAP + 0x4004;
const MTIME_LO    : u64 = CORE_LOCAL_INTERRUPT_MAP + 0xBFF8;
const MTIME_HI    : u64 = CORE_LOCAL_INTERRUPT_MAP + 0xBFFC;

pub fn init() -> Result<(), Error> {
  println!("time to ctx switch: {}", TIME_TO_CTX_SWITCH);
  write_time_cmp(TIME_TO_CTX_SWITCH);

  Ok(())
}

pub fn incr() -> Result<(), Error> {
  let mut time: u64 = 0;
  let (time_lo, time_hi) = read_mtime();

  time |= (time_lo & 0xFFFFFFFF) as u64;
  time |= ((time_hi & 0xFFFFFFFF) as u64) << 32;

  time = time.wrapping_add(TIME_TO_CTX_SWITCH);

  write_time_cmp(time);

  Ok(())
}

fn read_mtime() -> (u32, u32) {
  let time_lo_addr = MTIME_LO as *mut u32;
  let time_hi_addr = MTIME_HI as *mut u32;
  let time_lo: u32;
  let time_hi: u32;
  unsafe {
    time_lo = read_volatile(time_lo_addr);
    time_hi = read_volatile(time_hi_addr);
  }

  (time_lo, time_hi)
}

fn write_time_cmp(time: u64) -> () {
  let cmp_lo_addr = MTIME_CMP_LO as *mut u32;
  let cmp_hi_addr = MTIME_CMP_HI as *mut u32;
  unsafe {
    write_volatile(cmp_lo_addr, (time & 0xFFFFFFFF) as u32);
    write_volatile(cmp_lo_addr, ((time >> 32) & 0xFFFFFFFF) as u32);
  }
}
