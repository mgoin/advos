use core::fmt::Error;

pub mod uart;

pub struct Console;

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        Console::write(s)
    }
}

impl Console {
    pub fn write(s: &str) -> Result<(), Error> {
        for c in s.chars() {
            uart::writechar(c as u8);
        }

        Ok(())
    }

    pub fn read() -> Result<(), Error> {
        let mut buffer: [char; 256] = [' '; 256]; // fill the buffer with a temp value
        let mut next_char_index = 0;
        while let Some(c) = uart::readchar() {
            if c.is_ascii_control() {
                match c as char {
                    '\n' =>
                    {
                        buffer[next_char_index] = c as char;
                        Ok(())
                    },
                    _ =>
                    {
                        Err("Got invalid char!")
                    },
                }.unwrap();
            }
            buffer[next_char_index] = c as char;
            next_char_index += 1;
        }
        Ok(())
    }
}
