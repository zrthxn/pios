//! A panic handler that infinitely waits.
use core::panic::PanicInfo;

#[panic_handler]
fn panicinf(_info: &PanicInfo) -> ! {
  unimplemented!()
}