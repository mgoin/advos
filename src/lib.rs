//Michael Goin, Jacob Rutherford, Jonathan Ambrose
//2-5-2019
//This is the main library of the os. In this, init,
//readchar, and writechar are all functions for the UART.

#![feature(panic_info_message,allocator_api,asm,lang_items,compiler_builtins_lib)]
//We are not permitted to use the standard library since it isn't written for our operating system
#![no_std]
#![no_mangle]
#![allow(dead_code,unused_variables)]

//The eh_personality tells our program how to unwind. We aren't going to write that, so tell
//it to do nothing.
#[lang = "eh_personality"]
pub extern fn eh_personality() {}

//Abort will be used when panic can't
#[no_mangle]
fn abort() -> !
{
   loop {}
}

//Panic handler will execute whenever our rust code panics. -> ! means that this function won't return,
//so we have to make sure it doesn't.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    abort()
}

//Constants defined for the UART. The clock frequency
//is set with features during compilation

const CLOCK_FREQ: u64 = 65_000_000; // QEMU Frequency
const BAUD_RATE: u64 = 115_200;
const DIVISOR: u64 = (CLOCK_FREQ / BAUD_RATE) - 1;

const UART_ADDR: u64 = 0x1001_3000;
const TXDATA: u64 = UART_ADDR + 0x000; // Transmit data register
const RXDATA: u64 = UART_ADDR + 0x004; // Receive data register
const TXCTRL: u64 = UART_ADDR + 0x008; // Transmit control register
const RXCTRL: u64 = UART_ADDR + 0x00c; // Receive control register
const IE:     u64 = UART_ADDR + 0x010; // UART interrupt enable
const IP:     u64 = UART_ADDR + 0x014; // UART interrupt pending
const DIV:    u64 = UART_ADDR + 0x018; // Baud rate divisor

use core::ptr::{read_volatile, write_volatile};

//This initializes the UART by setting the transmit enable and read enable bits
//as well as the divisor

pub fn init() -> () {
    let div = DIV as *mut u32;
    let txctrl = TXCTRL as *mut u32;
    let rxctrl = RXCTRL as *mut u32;
    unsafe {
        // Write baud rate divisor to div register
        write_volatile(div, DIVISOR as u32 & 0x0000_FFFF);
        // Enable transmission by setting first bit in txctrl register
        write_volatile(txctrl, read_volatile(txctrl) | 1);
        // Enable receiving by setting first bit in rxctrl register
        write_volatile(rxctrl, read_volatile(rxctrl) | 1);
    }
}

//This function reads a single character if it is available
//It returns none if one is not available

pub fn readchar() -> Option<u8> {
    let rxdata = RXDATA as *mut u32;
    let r: u32;
    unsafe {
        r = read_volatile(rxdata);
    }
    // If the FIFO is empty, return nothing
    if r >> 31 == 1 { None }
    // Otherwise, return the char
    else { Some(r as u8) }
}

//This function writes a single char. It will block until
//the FIFO has space for it. Then it will write it

pub fn writechar(byte: u8) -> () {
    let txdata = TXDATA as *mut u32;
    unsafe {
        let mut t: u32;
        // Block until the write FIFO has space
        loop {
            t = read_volatile(txdata);
            if t >> 31 == 0 { break; }
        }
        // Write the char to txdata
        write_volatile(txdata, (t & 0x0000) | byte as u32);
    }
}

macro_rules! print {
    ($fmt:expr) => ( for c in $fmt.chars() {writechar(c as u8)} );
}

macro_rules! print_char {
    ($fmt:expr) => ( writechar($fmt as u8) );
}

macro_rules! println {
    () => ( print!("\n") );
    ($fmt:expr) => ( print!(concat!($fmt, "\n")) );
}

#[no_mangle]
fn main() {
    // Intialize UART
    init();

    // Test writing to stdout using UART
    println!("Hello world!");
    println!("This means we can write to stdout using UART!");

    loop {
        // Test reading from stdin using UART
        if let Some(c) = readchar() {
            print!("You wrote '");
            print_char!(c);
            println!("' to stdin.");
        }
    }
}

