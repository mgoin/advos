// Master clock frequency in hertz
pub const CLOCK_FREQ: u64 = 65_000_000;

// Address of the UART for mmio
pub const UART_ADDR: u64 = 0x1001_3000;

// Target baud rate for the UART
pub const BAUD_RATE: u64 = 115_200;

// Address of the CLINT for software and timer interrupts
pub const CORE_LOCAL_INTERRUPT_MAP: u64 = 0x0200_0000;
