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


/// Prints an info, with a newline.
#[macro_export]
macro_rules! info {
  ($string:expr) => ({
    #[allow(unused_imports)]
    use crate::time::interface::TimeManager;

    let timestamp = $crate::arch::time::time_manager().uptime();
    let timestamp_subsec_us = timestamp.subsec_micros();

    $crate::print::_print(format_args_nl!(
      concat!("[  {:>3}.{:03}{:03}] ", $string),
      timestamp.as_secs(),
      timestamp_subsec_us / 1_000,
      timestamp_subsec_us % 1_000
    ));
  });
  ($format_string:expr, $($arg:tt)*) => ({
    #[allow(unused_imports)]
    use crate::time::interface::TimeManager;

    let timestamp = $crate::arch::time::time_manager().uptime();
    let timestamp_subsec_us = timestamp.subsec_micros();

    $crate::print::_print(format_args_nl!(
      concat!("[  {:>3}.{:03}{:03}] ", $format_string),
      timestamp.as_secs(),
      timestamp_subsec_us / 1_000,
      timestamp_subsec_us % 1_000,
      $($arg)*
    ));
  })
}

/// Prints a warning, with a newline.
#[macro_export]
macro_rules! warn {
  ($string:expr) => ({
    #[allow(unused_imports)]
    use crate::time::interface::TimeManager;

    let timestamp = $crate::arch::time::time_manager().uptime();
    let timestamp_subsec_us = timestamp.subsec_micros();

    $crate::print::_print(format_args_nl!(
      concat!("[W {:>3}.{:03}{:03}] ", $string),
      timestamp.as_secs(),
      timestamp_subsec_us / 1_000,
      timestamp_subsec_us % 1_000
    ));
  });
  ($format_string:expr, $($arg:tt)*) => ({
    #[allow(unused_imports)]
    use crate::time::interface::TimeManager;

    let timestamp = $crate::arch::time::time_manager().uptime();
    let timestamp_subsec_us = timestamp.subsec_micros();

    $crate::print::_print(format_args_nl!(
      concat!("[W {:>3}.{:03}{:03}] ", $format_string),
      timestamp.as_secs(),
      timestamp_subsec_us / 1_000,
      timestamp_subsec_us % 1_000,
      $($arg)*
    ));
  })
}
