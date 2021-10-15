use crate::bsp;
use core::fmt;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
  use crate::console::interface::Interactive;
  bsp::serial::serial().write_fmt(args).unwrap();
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

/// Prints package name and version.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! printpkg {
  () => (
    $crate::print!(
      "[v] {} version {}\n",
      env!("CARGO_PKG_NAME"),
      env!("CARGO_PKG_VERSION")
    )
  );
}