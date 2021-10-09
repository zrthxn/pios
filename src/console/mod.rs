pub mod interface {
  use core::fmt;

  /// Console write functions.
  pub trait Writeable {
    /// Write a single character.
    fn write_char(&self, c: char);

    /// Write a Rust format string.
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

    /// Block until the last buffered character has been physically put on the TX wire.
    fn flush(&self);
  }

  /// Console read functions.
  pub trait Readable {
    /// Read a single character.
    fn read_char(&self) -> char {
      ' '
    }

    /// Clear RX buffers, if any.
    fn clear_rx(&self);
  }

  pub trait History {
    /// Return the number of characters written.
    fn chars_written(&self) -> usize {
      0
    }

    /// Return the number of characters read.
    fn chars_read(&self) -> usize {
      0
    }
  }

  pub trait Interactive = Readable + Writeable + History;
}