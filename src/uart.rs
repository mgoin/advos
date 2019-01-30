// hifive1 freq: 17,422,745
// qemu freq: 65,000,000
//
// transmit control register (txctrl) offset 0x008
// bit 0: txen: Transmit enable
// recieve control register (rxctrl) offset 0x00C
// bit 0: rxen: Recieve enable
//
// 1. Set divisor for baud rate
// 2. Enable transmission
// 3. Enable recieve
//
// Then make readchar -> u8
// 1. Read the register (use readvolatile)
// 2. Check if bit 31 is set

const CLOCK_FREQ: u64 = 65_000_000; // Hz
const BAUD_RATE: u64 = 115_200;
const DIVISOR: u64 = (CLOCK_FREQ / BAUD_RATE) - 1;

const UART_ADDR: u64 = 0x1001_3000;
const TXDATA: u64 = UART_ADDR + 0x000; // Transmit data register
const RXDATA: u64 = UART_ADDR + 0x004; // Recieve data register
const TXCTRL: u64 = UART_ADDR + 0x008; // Transmit control register
const RXCTRL: u64 = UART_ADDR + 0x00c; // Recieve control register
const IE:     u64 = UART_ADDR + 0x010; // UART interrupt enable
const IP:     u64 = UART_ADDR + 0x014; // UART interrupt pending
const DIV:    u64 = UART_ADDR + 0x018; // Baud rate divisor


#[no_mangle]
pub fn init() -> Result<(), ()> {
    let div = DIV as *mut u32;
    let txctrl = TXCTRL as *mut u32;
    let rxctrl = RXCTRL as *mut u32;

    unsafe {
        *div = DIVISOR as u32 & 0x0000_FFFF;
        *txctrl = *txctrl | 1;
        *rxctrl = *rxctrl | 1;
    }

    return Ok(());
}

pub fn readchar() -> Option<u8> {
    let rxdata = RXDATA as *mut u32;
    unsafe {
        // don't block in read
        if *rxdata >> 31 == 1 { None } 
        else { Some(*rxdata as u8) }
    }
}

pub fn writechar(byte: u8) -> Result<(), ()> {
    let txdata = TXDATA as *mut u32;
    unsafe {
        // reading txdata register is non-destructive so 
        // just have a loop running until we can write
        while *txdata >> 31 == 1 {}
        *txdata = (*txdata & 0x0000) | byte as u32;
    }
    Ok(())
}
