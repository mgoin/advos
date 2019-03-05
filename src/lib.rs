// Michael Goin, Jacob Rutherford, Jonathan Ambrose
// 2-13-2019
// This iteration of lib contains the print! and println! macros
// and tests these macros using the Console.

#![feature(panic_info_message,
           allocator_api,
           asm,
           lang_items,
           compiler_builtins_lib)]
// We are not permitted to use the standard library since it isn't written for
// our operating system
#![no_std]
#![no_mangle]
#![allow(dead_code, unused_variables)]

mod console;
mod global_constants;
mod lock;
mod memman;
mod trap;
mod utils;

use console::Console;
use core::fmt::Write;

use memman::MemManager;

#[cfg(feature = "testing")]
use utils::stackvec::StackVec;

// The print! macro will print a string by calling write!

#[macro_export]
macro_rules! print {
    ($fmt:expr) => {
        write!(Console, $fmt).unwrap();
    };
    ($fmt:expr, $($args:tt)*) => {
        write!(Console, "{}", format_args!($fmt, $($args)*)).unwrap();
    };
}

// The println! macro appends \r\n to the string and then calls
// the print! macro

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

// The eh_personality tells our program how to unwind. We aren't going to write
// that, so tell it to do nothing.
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

// Abort will be used when panic can't
#[no_mangle]
fn abort() -> ! {
    loop {}
}

// Panic handler will execute whenever our rust code panics. -> ! means that this
// function won't return, so we have to make sure it doesn't.
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

#[cfg(feature = "testing")]
fn test_mutex() -> () {
    let mut m = lock::Mutex::new();
    print!("Locking mutex...");
    m.lock();
    assert_eq!(m.get_state(), 1);
    println!("Success");
    print!("Trying to lock mutex again...");
    let res1 = m.try_lock();
    assert_eq!(res1, Some(false));
    println!("Success");
    print!("Unlocking mutex...");
    m.unlock();
    assert_eq!(m.get_state(), 0);
    println!("Success");
    print!("Trying to lock mutex again...");
    let res = m.try_lock();
    assert_eq!(res, Some(true));
    println!("Success");
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

#[cfg(feature = "testing")]
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

#[cfg(feature = "testing")]
fn test_memman() -> () {
    unsafe {
        // Allocate an 16 byte quantity

        let p = MemManager::kmalloc(16).unwrap();
        let pnt = p as *mut u32;
        *pnt = 12;
        assert_eq!(*pnt, 12);

        // Allocate an 8-byte quantity

        let pt = MemManager::kmalloc(8).unwrap();
        let pts = pt as *mut u32;
        assert!(pt > p);
        *pts = 8;
        assert_eq!(*pts, 8);

        // Allocate a 24 byte quantity

        let pt1 = MemManager::kmalloc(24).unwrap();
        assert!(pt1 > pt);
        let pt1s = pt1 as *mut u32;
        *pt1s = 4;
        assert_eq!(*pt1s, 4);

        // Free the middle quantity that is 8 bytes

        assert!(MemManager::kfree(pt).is_ok());

        // Allocate a 24 byte quantity to show that
        // it won't take the 8 byte quantity in the middle

        let pt24 = MemManager::kmalloc(24).unwrap();
        assert!(pt24 != pt);
        let pts = pt24 as *mut u32;
        *pts = 3;
        assert_eq!(*pts, 3);

        // Now show that a small enough block will take it

        let pt4 = MemManager::kmalloc(4).unwrap();
        assert!(pt4 == pt);
        let pt4s = pt4 as *mut u32;
        *pt4s = 3;
        assert_eq!(*pt4s, 3);

        // Free them all

        assert!(MemManager::kfree(p).is_ok());
        assert!(MemManager::kfree(pt1).is_ok());
        assert!(MemManager::kfree(pt24).is_ok());
        assert!(MemManager::kfree(pt).is_ok());

        // Show that fragmentation doesn't let this go at the front

        let pt = MemManager::kmalloc(24).unwrap();
        let pts = pt as *mut u32;
        *pts = 17;
        assert_eq!(*pts, 17);

        // Free it, coalesce, and show it will go at the front

        assert!(MemManager::kfree(pt).is_ok());
        MemManager::kcoalesce();
        let pt = MemManager::kmalloc(24).unwrap();
        assert_eq!(p, pt);

        let pts = pt as *mut u32;
        *pts = 14;
        assert_eq!(*pts, 14);

        assert!(MemManager::kfree(pt).is_ok());
    }
}

#[cfg(feature = "testing")]
fn test_stackvec() {
    let mut storage: [u32; 32] = [0u32; 32];
    let mut vec = stackvec!(&mut storage);

    assert_eq!(vec.buffer_size(), 32);
    assert_eq!(vec.size(), 0);

    vec.push(23).unwrap();
    assert_eq!(vec.size(), 1);
    vec.push(12).unwrap();
    assert_eq!(vec.size(), 2);

    assert_eq!(vec[0], 23);
    assert_eq!(vec[1], 12);


    let mut t = vec.pop().unwrap();
    assert_eq!(*t, 12);
    assert_eq!(vec.size(), 1);
    t = vec.pop().unwrap();
    assert_eq!(*t, 23);
    assert_eq!(vec.size(), 0);

    vec.push(1).unwrap();
    vec.push(2).unwrap();
    vec.push(3).unwrap();
    assert_eq!(vec.size(), 3);

    let mut i = vec.iter();
    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.next(), None);
}

#[cfg(feature = "testing")]
fn run_tests() {
    test_println();
    test_mutex();
    test_memman();
    test_stackvec();
}

#[no_mangle]
fn main() {
    unsafe {
        enable_interrupts();
    }
    println!("interrupts enabled");

    // Intialize UART for reading/writing
    print!("initializing UART...");
    console::uart::init().unwrap();
    println!("Done");

    print!("initializing MemManager...");
    MemManager::init();
    println!("Done");

    #[cfg(feature = "testing")]
    {
        run_tests();

        println!("testing interrupts");
        let clim = global_constants::CORE_LOCAL_INTERRUPT_MAP as *mut u32;
        let interrupt_mask: u32 = 0x008;
        println!("sending software interrupt");
        unsafe {
            core::ptr::write_volatile(clim, interrupt_mask);
        }

        println!("sending ecall");
        unsafe {
            asm!("ecall" ::::"volatile");
        }

        println!("\n\ntests finished, exit qemu\n\n");
        loop {}
    }

    print!("initializing system timer...");
    trap::timer::init().unwrap();
    println!("Done");

    println!("Type into the console:");
    loop {
        echo_from_console();
    }
}
