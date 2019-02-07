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
        let mut ret_str: [char; 256] = ['a'; 256]; // fill the buffer with a filler letter
        let mut next_char_index = 0;
        while let Some(c) = uart::readchar() {
            if c.is_ascii_control() {
                match c as char {
                    '\n' =>
                    {
                        ret_str[next_char_index] = c as char;
                        Ok(())
                    },
                    _ =>
                    {
                        Err("Got invalid char!")
                    },
                }.unwrap();
            }
            ret_str[next_char_index] = c as char;
            next_char_index += 1;
        }
        Ok(())
    }
}
