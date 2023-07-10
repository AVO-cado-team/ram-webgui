use std::cmp::min;
use std::io::{BufRead, Error, ErrorKind, Read, Result};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CustomReader {
    input: String,
}

impl CustomReader {
    pub fn new(input: &str) -> Self {
        let mut reader = Self::default();
        reader.set_input(input);
        reader
    }

    pub fn set_input(&mut self, input: &str) {
        // NOTE: there should be '\n' at the end of input, so be aware of that behavior
        self.input = input
            .split_whitespace()
            .fold(String::new(), |a, b| a + b + "\n");
    }
}

impl Read for CustomReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.input.is_empty() {
            //  TODO: wait for input
            return Err(Error::new(ErrorKind::UnexpectedEof, "No input available"));
        }
        let len = min(buf.len(), self.input.len());
        buf[..len].copy_from_slice(&self.input.as_bytes()[..len]);
        Ok(len)
    }
}

impl BufRead for CustomReader {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        if self.input.is_empty() {
            //  TODO: wait for input
            return Err(Error::new(ErrorKind::UnexpectedEof, "No input available"));
        }
        Ok(self.input.as_bytes())
    }

    fn consume(&mut self, amt: usize) {
        self.input.drain(..min(amt, self.input.len()));
    }
}
