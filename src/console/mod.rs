pub mod text;
pub mod font;

pub mod interface {
  use core::fmt;

  /// Console write functions.
  pub trait Write {
    /// Write a single character.
    fn write_char(&self, c: char);

    /// Write a Rust format string.
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

    /// Block until the last buffered character has been physically put on the TX wire.
    fn flush(&self);
  }

  /// Console read functions.
  pub trait Read {
    /// Read a single character.
    fn read_char(&self) -> char {
      ' '
    }

    /// Clear RX buffers, if any.
    fn clear_rx(&self);
  }

  pub trait Statistics {
    /// Return the number of characters written.
    fn chars_written(&self) -> usize {
      0
    }

    /// Return the number of characters read.
    fn chars_read(&self) -> usize {
      0
    }
  }

  pub trait Interactive = Read + Write + Statistics;
}

pub static Console: text::Text = text::Text::new();

/// Clears the console
#[macro_export]
macro_rules! clsc {
  () => (Console.clear());
}

/// Writes a char or string
#[macro_export]
macro_rules! cout {
  () => (Console.clear());
}

/// Reads a char or string
#[macro_export]
macro_rules! cin {
  () => (Console.clear());
}
