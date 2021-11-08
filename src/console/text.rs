use crate::console::font;
use crate::{info, println};
use crate::sync::{NullLock, interface::Mutex};
use crate::bsp::{GPU, devices::gpu::Glyph};

struct TextInner {
  buffer: [char; 4096],
  rx_index: usize,
  tx_index: usize,
}

impl TextInner {
  pub const fn new() -> Self {
    Self {
      buffer: ['\0'; 4096],
      rx_index: 0,
      tx_index: 0,
    }
  }

  fn clear(&mut self) {
    self.clear_rx();
    GPU.cls()
  }

  fn write_char(&mut self, c: char) {
    self.buffer[self.tx_index] = c;
    self.tx_index = self.tx_index + 1;
    self.flush()
  }

  fn write_str(&mut self, s: &str) {
    for c in s.chars() {
      self.buffer[self.tx_index] = c;
      self.tx_index = self.tx_index + 1;
    }

    self.flush()
  }

  fn write_fmt(&self, args: core::fmt::Arguments) -> core::fmt::Result {
    Ok(())
  }

  fn flush(&mut self) {
    self.clear();
    let mut loc: (u16, u16) = (16,16);

    for i in 0..self.tx_index {
      let ch = self.buffer[i];
      match ch {
        '\0' => {
          self.tx_index = i;
          break;
        },
        '\n' => {
          loc.1 = loc.1 + (8 + 4);
          loc.0 = 16;
        },
        _ => {
          GPU.write_glyph(loc, font::bitmap(ch));
          loc.0 = loc.0 + (8 + 2)
        }
      }      
    }
  }

  fn read_char(&mut self) -> char {
    self.rx_index = self.rx_index - 1;
    self.buffer[self.rx_index]
  }

  fn read_str(&mut self) { // -> &str {
    let mut _buf = self.tx_index;
    while self.buffer[_buf] != '\0' {
      _buf = _buf - 1;
    }
    self.rx_index = _buf;

    // vec is pretty much required
    let mut _str = ['\0'; 1024];
    for i in self.rx_index..self.tx_index {
      _str[i] = self.buffer[i];
    }

    // return _str;
  }

  fn clear_rx(&mut self) {
    self.rx_index = self.tx_index;
  }
}

pub struct Text {
  inner: NullLock<TextInner>
}

impl Text {
  pub const fn new() -> Self {
    Self {
      inner: NullLock::new(TextInner::new())
    }
  }

  pub fn clear(&self) {
    self.inner.lock(|inner| inner.clear())
  }

  pub fn write_str(&self, str: &str) {
    self.inner.lock(|inner| inner.write_str(str))
  }
}

impl super::interface::Write for Text {
  fn write_char(&self, c: char) {
    self.inner.lock(|inner| inner.write_char(c))
  }

  fn write_fmt(&self, args: core::fmt::Arguments) -> core::fmt::Result {
    self.inner.lock(|inner| inner.write_fmt(args))
  }

  fn flush(&self) {
    self.inner.lock(|inner| inner.flush())
  }
}

impl super::interface::Read for Text {
  fn read_char(&self) -> char {
    self.inner.lock(|inner| inner.read_char())
  }

  fn clear_rx(&self) {
    self.inner.lock(|inner| inner.clear_rx())
  }
}