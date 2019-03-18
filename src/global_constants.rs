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
