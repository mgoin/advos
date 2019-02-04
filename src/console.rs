use uart;

struct Console {
}

impl core::fmt::write for Console {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
    }
}
