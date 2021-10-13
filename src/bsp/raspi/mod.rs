mod cpu;
pub mod memory;
pub mod drivers;

use super::devices;

/// Global instances
pub static GPIO: devices::bcm2xxx::GPIO =
  unsafe { devices::bcm2xxx::GPIO::new(memory::map::mmio::GPIO_START) };

pub static UART: devices::bcm2xxx::PL011UART =
  unsafe { devices::bcm2xxx::PL011UART::new(memory::map::mmio::PL011_UART_START) };

/// Board identification.
pub fn board_name() -> &'static str {
  #[cfg(feature = "bsp_rpi3")]
  {
    "Raspberry Pi 3"
  }

  #[cfg(feature = "bsp_rpi4")]
  {
    "Raspberry Pi 4"
  }
}

pub mod serial;
pub use serial::*;