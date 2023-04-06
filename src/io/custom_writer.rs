use std::io::Write;

use yew::Callback;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CustomWriter {
  pub output: String,
  pub on_write: Option<Callback<String>>,
}

impl CustomWriter {
  pub fn new() -> Self {
    Self {
      output: String::new(),
      on_write: None,
    }
  }

  pub fn set_on_write(&mut self, on_write: Callback<String>) {
    self.on_write = Some(on_write);
  }

  pub fn write(&mut self, data: String) {
    self.output.push_str(&data);
    log::info!("called, calling: {:?} with {}", self.on_write, &data);
    if let Some(on_write) = &self.on_write {
      on_write.emit(data);
    }
  }
}

impl Write for CustomWriter {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    let data = String::from_utf8_lossy(buf).to_string();
    self.write(data);
    Ok(buf.len())
  }

  fn flush(&mut self) -> std::io::Result<()> {
    Ok(())
  }
}

