use crate::{bsp, console};
use core::fmt;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
  use console::interface::Writeable;

  bsp::console::console().write_fmt(args).unwrap();
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
  () => ($crate::print!("\n"));
  ($($arg:tt)*) => ({
    $crate::print::_print(format_args_nl!($($arg)*));
  })
}