// Implementation of mutex for locking
#[repr(align(4))] // Needs to be aligned for atomic instr
pub struct Mutex {
  state: u8,
}

extern "C" {
  fn mutex_lock(state: *mut u8) -> ();
}

impl Mutex {
  // Creates free (unlocked) mutex
  pub fn new() -> Mutex { 
    Mutex { state: 0 }
  }
  
  // Tries to lock the mutex, blocking until it can do so
  pub fn lock(&mut self) {
    unsafe {
      /*
      asm!("
          li t0, 1                  # Initialize swap value
        again:
          amoswap.w.aq t0, t0, ($0) # Attempt to acquire lock
          bnez t0, again            # Retry if held"
        :: "r"(&self.state) : "t0" : "volatile");
        */
      mutex_lock(&mut self.state);
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

pub struct Barrier {
  arrive_counter: u32,  // How many procs have entered, 0 at start
  leave_counter: u32,   // How many procs have exited, N at start
  flag: u32,
  mutex: Mutex,
}

impl Barrier {
  pub fn new(n: u32) -> Barrier {
    Barrier { 
      arrive_counter: 0, 
      leave_counter: n, 
      flag: 0, 
      mutex: Mutex::new() }
  }

  pub fn barrier(&mut self, n: u32) {
    self.mutex.lock();
    if self.leave_counter == n {
      if self.arrive_counter == 0 { // No other threads in barrier
        self.flag = 0; // First arriver clears flag
      }
      else {
        self.mutex.unlock();
        // Wait for all to leave before clearing
        while self.leave_counter != n {}
        self.mutex.lock();
        self.flag = 0; // First arriver clears flag
      }
    }
    self.arrive_counter += 1;
    let arrived = self.arrive_counter;
    self.mutex.unlock();
    if arrived == n { // Last arriver sets flag
      self.arrive_counter = 0;
      self.leave_counter = 1;
      self.flag = 1;
    }
    else {
      while self.flag == 0 {} // Wait for flag
      self.mutex.lock();
      self.leave_counter += 1;
      self.mutex.unlock();
    }
  }
}

pub struct Semaphore {
  n: i32,
  count_mutex: Mutex, // Unlocked initially
  queue_mutex: Mutex, // Locked initially
}

impl Semaphore {
  pub fn new(size: u32) -> Semaphore{
    let mut s = Semaphore { 
      n: size as i32, 
      count_mutex: Mutex::new(),
      queue_mutex: Mutex::new() };
    s.queue_mutex.lock();
    return s;
  }

  pub fn wait(&mut self) {
    self.count_mutex.lock();
    self.n -= 1;
    if self.n < 0 {
      self.count_mutex.unlock();
      self.queue_mutex.lock();   // Wait
    }
    self.count_mutex.unlock();   // Unlock signal's lock
  }

  pub fn signal(&mut self) {
    self.count_mutex.lock();
    self.n += 1;

    if self.n <= 0 {
      self.queue_mutex.unlock();  // Leave count_mutex locked
    }
    else {
      self.count_mutex.unlock();
    }
  }
}
