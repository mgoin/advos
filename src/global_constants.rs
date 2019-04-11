// Master clock frequency in hertz
#[cfg(not(feature = "board"))]
pub const CLOCK_FREQ: u64 = 10_000_000;

#[cfg(feature = "board")]
pub const CLOCK_FREQ: u64 = 17_422_745;

// Address of the UART for mmio
pub const UART_ADDR: u64 = 0x1001_3000;

// Target baud rate for the UART
pub const BAUD_RATE: u64 = 115_200;

// Address of the CLINT for software and timer interrupts
pub const CORE_LOCAL_INTERRUPT_MAP: u64 = 0x0200_0000;

// Max number of processes that can run at one time
pub const MAX_PROC_COUNT: usize = 16;

// Allocated size for one process
pub const PROC_ALLOC_SIZE: usize = 1 << 8;

// Number of CPU registers
pub const NUM_CPU_REGISTERS: usize = 32;

// Magic number in superblock to check validity
pub const SUPERBLOCK_MAGIC: u32 = 0xef53;

// Size of blocks for our filesystem
pub const DEVICE_BLOCK_SIZE: u32 = 512;