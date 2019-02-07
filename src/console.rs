use core::fmt::Error;

pub mod uart;

pub struct Console;

impl core::fmt::write for Console {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
    }
}
