// hifive1 freq: 17,422,745
// qemu: 65,000,000
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
//

pub fn init() -> Result<(), ()> {
    
    return Err(());
}

pub fn readchar() -> u8 {
    // don't block in read
    let recieve_data: u32 = 0;
    let result: u8 = 0;

    // read rxdata register into u32
    // offset 0x004
    return result;
}

pub fn writechar(byte: u8) -> Result<(), ()> {
    // block in write
    
    // reading txdata register is non-destructive so 
    // just have a loop running until the write goes through
    return Ok(());
}
