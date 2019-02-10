//Michael Goin, Jacob Rutherford, and Jonathan Ambrose
//2-15-2019
//This code implements a console. The read function will receive characters up until
//a point that it receives an invalid control character or a '\r'. If
//it gets a '\r', it returns the buffer. Otherwise, it returns a None.
//The Write trait is also implemented for the Console to allow it use fmt
//to handle the formatting for the print! and println! macros.

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
        let mut arrow_count = 0;

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
                    Console::write_char(c);
                    match c {

                        // Carriage return is given when the enter key is
                        // pressed, which is the trigger to return the buffer
                        // to the caller.

                        '\r' => return Some(buffer),

                        // backspace (\u{0008} or ascii 0x08) and
                        // delete (\u{007f} or ascii 0x7f) character

                        '\u{8}'|'\u{7f}' => {

                            //This if keeps us from getting a negative index

                            if next_char_index != 0 {

                                //If we've moved the arrows, we need to account for that.
                                //We'll shift the buffer internally

                                if arrow_count != 0 {
                                    let mut move_count = arrow_count;
                                    let mut next_index = next_char_index;
                                    while move_count != 0 {
                                        buffer[next_index - 1] = buffer[next_index];
                                        move_count -= 1;
                                        next_index += 1;
                                    }
                                }

                                //Makes the new last character '\0'

                                buffer[next_char_index + arrow_count - 1] = '\0';
                                next_char_index -= 1;

                                //Here we essentially rewrite the buffer to the screen
                                //First we clear the line with spaces. Then we reprint
                                //the buffer.

                                Console::write_char('\r');
                                for i in 1..next_char_index+arrow_count+2 {
                                    Console::write_char(' ');
                                }

                                Console::write_char('\r');
                                for c in buffer.iter() {
                                    Console::write_char(*c);
                                }

                                //Here we rewrite the screen so our cursor is in the proper
                                //position if we've moved around

                                if arrow_count != 0 {
                                    Console::write_char('\r');
                                    for i in 0..next_char_index {
                                        Console::write_char(buffer[i]);
                                    }
                                }
                            }
                        },

                        //This captures an arrow key. This is followed by a '[' and
                        //then by a letter. The letter corresponds to the direction

                        '\u{1b}' => {
                            let mut count = 0;

                            //Here we read those next two characters

                            while count < 2 {
                                if let Some(bb) = uart::readchar() {
                                    let cc = bb as char;

                                    //A 'D' is a left arrow. Move left if we aren't
                                    //at 0.

                                    if cc == 'D' {
                                        if next_char_index != 0 {
                                            next_char_index -= 1;
                                            arrow_count += 1;
                                        }
                                    }

                                    //A 'C' is a right arrow. Move right if we aren't
                                    //at the end of the current buffer.

                                    else if cc == 'C' {
                                        if buffer[next_char_index] != '\0' {
                                            next_char_index += 1;
                                            arrow_count -= 1;
                                        }
                                    }
                                    count += 1;
                                }
                            }

                            //Here we rewrite the screen to show proper cursor position.
                            //The ' ' is there because a character must be printed after
                            //the arrow to print the first character of the buffer. It is
                            //odd, but it doesn't hurt anything to print it to get that
                            //first character.

                            Console::write_char('\r');
                            Console::write_char(' ');
                            for i in 0..next_char_index {
                                Console::write_char(buffer[i]);
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
                //end of the buffer. If we aren't, we print the character, add it
                //to the buffer, and increment the next index.

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
