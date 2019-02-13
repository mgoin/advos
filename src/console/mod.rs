//Michael Goin, Jacob Rutherford, and Jonathan Ambrose
//2-15-2019
//This code implements a console. The read function will receive characters up
//until a point that it receives an invalid control character or a '\r'. If it
//gets a '\r', it returns the buffer. Otherwise, it returns a None.  The Write
//trait is also implemented for the Console to allow it use fmt to handle the
//formatting for the print! and println! macros.

use core::fmt::Error;

pub mod uart;

pub struct Console;

const BUFFER_LENGTH : usize = 256;

//This implements the Write trait for the console

impl core::fmt::Write for Console {

    //The Write trait simply must use a write_str function.
    //For our implementation, we passed this off to a function
    //internal to the console

    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        Console::write(s)
    }
}

impl Console {

    //The write function simply takes a string and writes its
    //characters individually via the writechar function of the
    //UART

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

    //The read function of the console allows one to read continually
    //until a new line is found

    pub fn read() -> Option<[char; BUFFER_LENGTH]> {

        // fill the buffer with a temp value

        let mut buffer: [char; BUFFER_LENGTH] = ['\0';BUFFER_LENGTH];
        let mut next_char_index = 0;

        // Read will buffer input until the user hits enter

        loop {

            // If we read a control character, we print the character.
            // If it was '\r', we return the buffer. If it was an arrow
            // key, we move the cursor and the position in the buffer.
            // Otherwise, it is an uncaptured control sequence. In this
            // case, we print a '\n' and return a None.

            if let Some(b) = uart::readchar() {
                let c = b as char;
                if c.is_control() {
                    match c {

                        // Carriage return is given when the enter key is
                        // pressed, which is the trigger to return the buffer
                        // to the caller.

                        '\r' => {
                            return Some(buffer);
                        },

                        // backspace (\u{0008} or ascii 0x08) and
                        // delete (\u{007f} or ascii 0x7f) character

                        '\u{8}'|'\u{7f}' => {

                            //This if keeps us from getting a negative index

                            if next_char_index != 0 {

                                //Makes the new last character '\0'

                                buffer[next_char_index - 1] = '\0';
                                next_char_index -= 1;

                                //Here we essentially rewrite the buffer to the
                                //screen. First we clear the line with spaces.
                                //Then we reprint the buffer.

                                Console::write_char('\r');
                                for i in 0..next_char_index+1 {
                                    Console::write_char(' ');
                                }

                                Console::write_char('\r');
                                for c in buffer.iter() {
                                    Console::write_char(*c);
                                }
                            }
                        },

                        // Unhandled control charaters

                        _ => {
                            Console::write_char('\n');
                            return None;
                        },
                    }
                }

                //If it isn't a control character, we make sure we aren't at the
                //end of the buffer. If we aren't, we print the character, add
                //it to the buffer, and increment the next index.

                else {
                    if next_char_index != BUFFER_LENGTH {
                        Console::write_char(c);
                        buffer[next_char_index] = c as char;
                        next_char_index += 1;
                    }
                }
            }
        }
    }
}
