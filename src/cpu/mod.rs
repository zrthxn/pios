use cortex_a::asm;

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn halt() -> ! {
  loop {
    asm::wfe()
  }
}

pub use cortex_a::asm::wfe;
pub use cortex_a::asm::nop;

#[cfg(feature = "bsp_rpi3")]
#[inline(always)]
pub fn spin_cycles(n: usize) {
  for _ in 0..n {
    nop();
  }
}