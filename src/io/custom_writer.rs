use std::io::Write;

use yew::Callback;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CustomWriter {
  pub on_write: Callback<String>,
}

impl CustomWriter {
  pub fn new(on_write: Callback<String>) -> Self {
    Self { on_write }
  }
}

impl Write for CustomWriter {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    let data = String::from_utf8_lossy(buf).to_string();
    self.on_write.emit(data);
    Ok(buf.len())
  }

  fn flush(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}
