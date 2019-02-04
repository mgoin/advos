#![feature(panic_info_message,allocator_api,asm,lang_items,compiler_builtins_lib)]
//We are not permitted to use the standard library since it isn't written for our operating system
#![no_std]
#![no_mangle]
#![allow(dead_code,unused_variables)]

mod uart;

macro_rules! print {
    ($fmt:expr) => ( for c in $fmt.chars() {uart::writechar(c as u8)} );
}

macro_rules! print_char {
    ($fmt:expr) => ( uart::writechar($fmt as u8) );
}

macro_rules! println {
    () => ( print!("\r\n") );
    ($fmt:expr) => ( print!(concat!($fmt, "\r\n")) );
//	($fmt:expr, $( $x:expr ),+) => (print!(concat!($fmt, "\r\n"), $($x)+));
}

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

#[no_mangle]
fn main() {
    // Intialize UART for reading/writing
    uart::init();

    println!("Hello world!");

    // loop {
    //     if let Some(c) = uart::readchar() {
    //         println!("Some");
    //     }
    // }
}

