//Michael Goin, Jacob Rutherford, Jonathan Ambrose
//2-13-2019
//This iteration of lib contains the print! and println! macros
//and tests these macros using the Console.

#![feature(panic_info_message,
           allocator_api,
           asm,
           lang_items,
           compiler_builtins_lib)]
//We are not permitted to use the standard library since it isn't written for
//our operating system
#![no_std]
#![no_mangle]
#![allow(dead_code, unused_variables)]

mod console;
mod global_constants;
mod lock;
mod memman;
mod trap;

use console::Console;
use core::fmt::Write;
use memman::MemManager;

//The print! macro will print a string by calling write!

#[macro_export]
macro_rules! print {
    ($fmt:expr) => {
        write!(Console, $fmt).unwrap();
    };
    ($fmt:expr, $($args:tt)*) => {
        write!(Console, "{}", format_args!($fmt, $($args)*)).unwrap();
    };
}

//The println! macro appends \r\n to the string and then calls
//the print! macro

#[macro_export]
macro_rules! println {
    () => ( print!("\r\n") );
    ($fmt:expr) => { print!(concat!($fmt, "\r\n")); };
    ($fmt:expr, $($args:tt)*) => {
        print!("{}", format_args!(concat!($fmt, "\r\n"), $($args)*))
    };
}

extern "C" {
    fn enable_interrupts() -> ();
    static HEAP_START: *const u32;
    static HEAP_END: *const u32;
}

//The eh_personality tells our program how to unwind. We aren't going to write
//that, so tell it to do nothing.
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

//Abort will be used when panic can't
#[no_mangle]
fn abort() -> ! {
    loop {}
}

//Panic handler will execute whenever our rust code panics. -> ! means that this
//function won't return, so we have to make sure it doesn't.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        println!("PANIC in file {}: line {} column {}",
                 loc.file(),
                 loc.line(),
                 loc.column());
    }
    abort()
}

fn test_mutex() -> () {
    let mut m = lock::Mutex::new();
    println!("Locking mutex...");
    m.lock();
    println!("Unlocking mutex...");
    m.unlock();
    println!("Locking mutex again...");
    m.lock();
}

fn echo_from_console() -> () {
    if let Some(s) = console::Console::read() {
        print!("\r\nread \"");
        for c in s.iter() {
            print!("{}", c);
        }
        println!("\" from uart");
    }
}

fn test_println() -> () {
    println!();
    println!("Test lines: ");
    println!("  Lowercase Hex: 15 = {:x}", 15);
    println!("  Uppercase Hex: 26 = {:X}", 26);
    println!("  Named References: for hello=7, reference hello yields {hello}",
             hello = 7);
    println!("  Octal: 12 = {:o}", 12);
    println!("  Formatted Double: 1.23456 of width 3 is {:.3}", 1.23456);
    println!("  Formatted Int: 42 of width 4 with leading zeroes is {:04}",
             42);
    println!();
}

#[no_mangle]
fn test_memman() -> () {
    unsafe {
        MemManager::init();

        //Allocate an 16 byte quantity

        let p = MemManager::kmalloc(16).unwrap();
        let pnt = p as *mut u32;
        println!("Allocated 16 bytes at Address {:}", p);
        println!("Value from pnt {:}", *pnt);
        *pnt = 12;
        println!("Value from pnt {:}", *pnt);
        println!();

        //Allocate an 8-byte quantity

        let pt = MemManager::kmalloc(8).unwrap();
        let pts = pt as *mut u32;
        println!("Allocated 8 bytes at Address {:}", pt);
        println!("Value from pnt {:}", *pts);
        *pts = 8;
        println!("Value from pnt {:}", *pts);
        println!();

        //Allocate a 24 byte quantity

        let pt1 = MemManager::kmalloc(24).unwrap();
        let pt1s = pt1 as *mut u32;
        println!("Allocated 24 bytes at Address {:}", pt1);
        println!("Value from pnt {:}", *pt1s);
        *pt1s = 4;
        println!("Value from pnt {:}", *pt1s);
        println!();

        //Free the middle quantity that is 8 bytes

        MemManager::kfree(pt).unwrap();
        println!("Freed Address {:}", pt);
        println!();

        //Allocate a 24 byte quantity to show that
        //it won't take the 8 byte quantity in the middle

        let pt = MemManager::kmalloc(24).unwrap();
        let pts = pt as *mut u32;
        println!("Allocated 24 bytes at Address {:}", pt);
        println!("Value from pnt {:}", *pts);
        *pts = 3;
        println!("Value from pnt {:}", *pts);
        println!();

        //Now show that a small enough block will take it

        let pt2 = MemManager::kmalloc(4).unwrap();
        let pt2s = pt2 as *mut u32;
        println!("Allocated 4 bytes at Address {:}", pt2);
        println!("Value from pnt {:}", *pt2s);
        *pt2s = 3;
        println!("Value from pnt {:}", *pt2s);
        println!();

        //Free them all

        MemManager::kfree(p).unwrap();
        MemManager::kfree(pt1).unwrap();
        MemManager::kfree(pt2).unwrap();
        MemManager::kfree(pt).unwrap();
        println!("Freed Address {:}", p);
        println!("Freed Address {:}", pt);
        println!("Freed Address {:}", pt1);
        println!("Freed Address {:}", pt2);
        println!();

        //Show that fragmentation doesn't let this go at the front

        let pt = MemManager::kmalloc(24).unwrap();
        let pts = pt as *mut u32;
        println!("Allocated 24 bytes at Address {:}", pt);
        println!("Value from pnt {:}", *pts);
        *pts = 17;
        println!("Value from pnt {:}", *pts);
        println!();

        //Free it, coalesce, and show it will go at the front

        MemManager::kfree(pt).unwrap();
        println!("Freed Address {:}", pt);
        println!();
        MemManager::kcoalesce();
        println!("Coalesced Free List");
        println!();
        let pt = MemManager::kmalloc(24).unwrap();
        let pts = pt as *mut u32;
        println!("Allocated 24 bytes at Address {:}", pt);
        println!("Value from pnt {:}", *pts);
        *pts = 14;
        println!("Value from pnt {:}", *pts);
        println!();

        MemManager::kfree(pt).unwrap();
        println!("Freed Address {:}", pt);
        println!();
    }
}

#[no_mangle]
fn main() {
    unsafe {
        enable_interrupts();
    }
    println!("interrupts enabled");

    // Intialize UART for reading/writing
    console::uart::init().unwrap();

    // Test lines for formatting with println!
    test_println();

    // Test mutex locking and unlocking
    test_mutex();

    // Test the Memory Manager
    test_memman();

    let clim = global_constants::CORE_LOCAL_INTERRUPT_MAP as *mut u32;
    let interrupt_mask: u32 = 0x008;

    println!("sending software interrupt");
    unsafe {
        core::ptr::write_volatile(clim, interrupt_mask);
    }

    println!("timer initialized");
    trap::timer::init().unwrap();

    loop {
        echo_from_console();
    }
}
