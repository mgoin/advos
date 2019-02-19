// Implementation of mutex for locking
#[repr(align(4))] // Needs to be aligned for atomic instr
pub struct Mutex {
  state: u8,
}

impl Mutex {
  // Creates free (unlocked) mutex
  pub fn new() -> Mutex { 
    Mutex { state: 0 }
  }
  
  // Tries to lock the mutex, blocking until it can do so
  pub fn lock(&mut self) {
    unsafe {
      asm!("
          li t0, 1                  # Initialize swap value
        again:
          amoswap.w.aq t0, t0, ($0) # Attempt to acquire lock
          bnez t0, again            # Retry if held"
        :: "r"(&self.state) : "t0" : "volatile");
    }
  }
  
  // Unlocks the mutex
  pub fn unlock(&mut self) {
    unsafe {
      // Release lock by storing 0
      asm!("amoswap.w.rl zero, zero, ($0)" :: "r"(&self.state) :: "volatile");
    }
  }
}