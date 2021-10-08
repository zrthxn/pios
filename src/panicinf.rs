//! A panic handler that infinitely waits.
use crate::{cpu, println};
use core::panic::PanicInfo;

#[panic_handler]
fn panicinf(_info: &PanicInfo) -> ! {
  if let Some(args) = _info.message() {
    println!("\n[kernel panic!] Spooked by {}", args);
  } else {
    println!("\n[kernel panic!]");
  }

  cpu::halt()
}