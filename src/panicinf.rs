//! A panic handler that infinitely waits.
use crate::{bsp, cpu};
use core::{fmt, panic::PanicInfo};

fn _panic_print(args: fmt::Arguments) {
  // use fmt::Write;
  // unsafe { bsp::raspi::panic_serial().write_fmt(args).unwrap() };
  use crate::console::interface::Interactive;
  bsp::qemu::serial().write_fmt(args).unwrap();
}

/// Prints with a newline - only use from the panic handler.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! panic_println {
  ($($arg:tt)*) => ({
    _panic_print(format_args_nl!($($arg)*));
  })
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  if let Some(args) = _info.message() {
    panic_println!("\n[kernel panic!] Spooked by {}", args);
  } else {
    panic_println!("\n[kernel panic!]");
  }

  cpu::halt()
}