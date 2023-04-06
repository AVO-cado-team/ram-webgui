use std::io::{Error, ErrorKind, Read};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CustomReader {
  input: Option<String>,
}

impl CustomReader {
  pub fn new() -> Self {
    Self { input: None }
  }

  pub fn set_input(&mut self, input: String) {
    self.input = Some(input);
  }
}

impl Read for CustomReader {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    match self.input.as_ref() {
      Some(input) => {
        log::info!("Input: {:?}", input);
        let bytes = input.as_bytes();
        let len = std::cmp::min(buf.len(), bytes.len());
        buf[..len].copy_from_slice(&bytes[..len]);
        self.input = None;
        Ok(len)
      }
      None => Err(Error::new(ErrorKind::UnexpectedEof, "No input available")),
    }
  }
}
