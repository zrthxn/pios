use crate::{console, sync::NullLock, sync::interface::Mutex};
use core::fmt;

/// A mystical, magical device for generating QEMU output out of the void.
pub struct QEMUOutput {
  chars_written: usize,
}

impl QEMUOutput {
  pub fn new() -> Self {
    Self {
      chars_written: 0
    }
  }

  pub fn write_char(&mut self, c: char) {
    unsafe {
      core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
    }

    self.chars_written += 1;
  }
}

impl fmt::Write for QEMUOutput {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for c in s.chars() {
      // Convert LF to CRLF
      if c == '\n' {
        self.write_char('\r')
      }

      self.write_char(c);
    }

    Ok(())
  }
}


/// Globally usable safe QEMU output
pub struct SafeQEMUOutput {
  inner: NullLock<QEMUOutput>
}

impl SafeQEMUOutput {
  pub fn new() -> Self {
    Self {
      inner: NullLock::new(QEMUOutput::new())
    }
  }
}

/// Passthrough of `args` to the `core::fmt::Write` implementation,
/// but guarded by a Mutex to serialize access.
impl console::interface::Writeable for SafeQEMUOutput {
  fn write_char(&self, c: char) {
    unimplemented!()    
  }

  fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
    self.inner.lock(|inner: &mut QEMUOutput| fmt::Write::write_fmt(inner, args))
  }

  fn flush(&self) {
    unimplemented!()
  }
}

impl console::interface::History for SafeQEMUOutput {
  fn chars_written(&self) -> usize {
    self.inner.lock(|inner: &mut QEMUOutput| inner.chars_written)
  }
}

/// Static variable to access globally usable safe QEMU output.
/// 
/// Accessed via `serial` method to get a global reference. 
static QEMU: SafeQEMUOutput = SafeQEMUOutput {
  inner: NullLock::new(
    QEMUOutput {
      chars_written: 0
    }
  )
};

/// Return a reference to the console.
pub fn serial() -> &'static SafeQEMUOutput {
  &QEMU
}