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

    // Function to help with debugging printable ascii characters
    fn write_char(c: char) -> () {
        uart::writechar(c as u8);
    }

    // Function to help with debugging non-printable ascii characters with
    // char.escape_debug()
    fn write_unicode(u: core::char::EscapeDebug) -> () {
        for c in u {
            uart::writechar(c as u8);
        }
    }

    pub fn read() -> Option<[char; 256]> {
        // fill the buffer with a temp value
        let mut buffer: [char; 256] = ['\0';256];
        let mut next_char_index = 0;

        // Read will buffer input until the user hits enter
        loop {
            if let Some(b) = uart::readchar() {
                let c = b as char;
                Console::write_char(c);
                if c.is_control() {
                    match c {
                        // Carriage return is given when the enter key is
                        // pressed, which is the trigger to return the buffer
                        // to the caller.
                        '\r' => return Some(buffer),

                        // backspace (\u{0008} or ascii 0x08) and
                        // delete (\u{007f} or ascii 0x7f) character
                        '\u{8}'|'\u{7f}' => {
                            buffer[next_char_index - 1] = '\0';
                            next_char_index -= 1;
                            Console::write_char('\r');
                            for c in buffer.iter() {
                                Console::write_char(*c);
                            }
                        },

                        // Unhandled control charaters
                        _ => {
                            return None;
                        },
                    }
                }
                else {
                    buffer[next_char_index] = c as char;
                    next_char_index += 1;
                }
            }
        }
    }
}
