pub mod interface {
  use core::fmt;

  pub trait Writeable {
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    // fn flush(&self, args: fmt::Arguments) -> fmt::Result;
  }

  // pub trait Readable {
  //   // Unimplemented
  // }

  pub trait History {
    fn chars_written(&self) -> usize;
  }

  pub trait Interactive = Writeable + History;
}