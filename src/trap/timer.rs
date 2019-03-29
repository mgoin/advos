use crate::global_constants::{CLOCK_FREQ, CORE_LOCAL_INTERRUPT_MAP};
use core::fmt::Error;
use core::ptr::{read_volatile, write_volatile};

const CTX_PER_SECOND: u64 = 1;
const TIME_TO_CTX_SWITCH: u32 = (CLOCK_FREQ / CTX_PER_SECOND) as u32;

const MTIME_CMP_LO: u64 = CORE_LOCAL_INTERRUPT_MAP + 0x4000;
const MTIME_CMP_HI: u64 = CORE_LOCAL_INTERRUPT_MAP + 0x4004;
const MTIME_LO: u64 = CORE_LOCAL_INTERRUPT_MAP + 0xBFF8;
const MTIME_HI: u64 = CORE_LOCAL_INTERRUPT_MAP + 0xBFFC;

pub fn init() -> Result<(), Error> {
    incr()
}

pub fn incr() -> Result<(), Error> {
    let (mut time_lo, mut time_hi) = read_mtime();

    let prev_time = time_lo;
    time_lo = time_lo.wrapping_add(TIME_TO_CTX_SWITCH);

    if time_lo < prev_time {
        time_hi += 1;
    }

    write_time_cmp(time_lo, time_hi);

    Ok(())
}

// Returns the current time
pub fn get_current_time() -> u64 {
    let (time_lo, time_hi) = read_mtime();
    return ((time_hi as u64) << 32) + (time_lo as u64);
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

fn write_time_cmp(time_lo: u32, time_hi: u32) -> () {
    let cmp_lo_addr = MTIME_CMP_LO as *mut u32;
    let cmp_hi_addr = MTIME_CMP_HI as *mut u32;
    unsafe {
        write_volatile(cmp_lo_addr, time_lo);
        write_volatile(cmp_hi_addr, time_hi);
    }
}
