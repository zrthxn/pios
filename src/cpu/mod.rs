pub mod exception;
pub mod time;

use cortex_a::asm;

pub use asm::nop;

#[cfg(feature = "bsp_rpi3")]
#[inline(always)]
pub fn spin_cycles(n: usize) {
  for _ in 0..n {
    asm::nop();
  }
}

#[inline(always)]
pub fn halt() -> ! {
  loop {
    asm::wfe()
  }
}