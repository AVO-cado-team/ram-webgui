use std::io::{BufRead, Error, ErrorKind, Read};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CustomReader {
  input: String,
}

impl CustomReader {
  pub fn new() -> Self {
    Self {
      input: String::new(),
    }
  }

  pub fn set_input(&mut self, input: String) {
    self.input = input;
  }
}

impl Read for CustomReader {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    if self.input.is_empty() {
      //  TODO: wait for input
      return Err(Error::new(ErrorKind::UnexpectedEof, "No input available"));
    }
    let len = std::cmp::min(buf.len(), self.input.len());
    buf[..len].copy_from_slice(&self.input.as_bytes()[..len]);
    Ok(len)
  }
}

impl BufRead for CustomReader {
  fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
    if self.input.is_empty() {
      //  TODO: wait for input
      return Err(Error::new(ErrorKind::UnexpectedEof, "No input available"));
    }
    Ok(self.input.as_bytes())
  }

  fn consume(&mut self, amt: usize) {
    self.input.drain(..std::cmp::min(amt, self.input.len()));
  }
}
