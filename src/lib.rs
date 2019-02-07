#![feature(panic_info_message,allocator_api,asm,lang_items,compiler_builtins_lib)]
//We are not permitted to use the standard library since it isn't written for our operating system
#![no_std]
#![no_mangle]
#![allow(dead_code,unused_variables)]

mod console;

use console::Console;
use core::fmt::Write;

macro_rules! print {
    ($fmt:expr) =>
    {
        write!(Console, $fmt).unwrap();
    };
    ($fmt:expr, $($args:tt)*) =>
    {
        write!(Console, "{}", format_args!($fmt, $($args)*)).unwrap();
    };
}

macro_rules! println {
    () => ( print!("\n") );
    ($fmt:expr) =>
    {
        print!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($args:tt)*) =>
    { 
        print!("{}", format_args!(concat!($fmt, "\n"), $($args)*))
    };
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
    console::uart::init();

    println!("Hello world!");
    println!("test = {} and next test = {}", 1234, 98676);

    loop {
        if let Some(c) = console::uart::readchar() {
            print!("read ");
            print!("{}", c as char);
            println!(" from uart");
        }
    }
}
