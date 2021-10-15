mod cpu;
pub mod memory;
pub mod drivers;
pub mod video;
pub mod serial;

pub use memory::map::mmio;
use super::devices;

/// Global instances
pub static GPIO: devices::gpio::GPIO = unsafe { 
  devices::gpio::GPIO::new(mmio::GPIO_START) 
};

pub static UART: devices::uart::PL011UART = unsafe { 
  devices::uart::PL011UART::new(mmio::PL011_UART_START) 
};

pub static MAILBOX: devices::mailbox::MAILBOX = unsafe { 
  devices::mailbox::MAILBOX::new(mmio::MAILBOX_START) 
};

pub static GPU: devices::vc::GPU = unsafe { 
  devices::vc::GPU::new() 
};

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