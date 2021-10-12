use crate::console;
use crate::bsp::devices;
use core::fmt;

use super::{memory, UART};

/// In case of a panic, the panic handler uses this function to take a last shot at printing
/// something before the system is halted.
///
/// We try to init panic-versions of the GPIO and the UART. The panic versions are not protected
/// with synchronization primitives, which increases chances that we get to print something, even
/// when the kernel's default GPIO or UART instances happen to be locked at the time of the panic.
///
/// # Safety
///
/// - Use only for printing during a panic.
#[allow(dead_code)]
pub unsafe fn panic_serial() -> impl fmt::Write {
  let mut panic_gpio = devices::bcm2xxx::PanicGPIO::new(memory::map::mmio::GPIO_START);
  let mut panic_uart = devices::bcm2xxx::PanicUart::new(memory::map::mmio::PL011_UART_START);

  panic_gpio.map_pl011_uart();
  panic_uart.init();
  panic_uart
}

pub fn serial() -> &'static impl console::interface::Interactive {
  &UART
}