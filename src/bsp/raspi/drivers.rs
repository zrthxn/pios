use crate::driver;
use driver::interface::DeviceDriver;

/// Device Driver Manager type.
struct BSPDriverManager {
  drivers: [ &'static (dyn DeviceDriver + Sync); 4 ]
}

impl driver::interface::DriverManager for BSPDriverManager {
  fn list_drivers(&self) -> &[ &'static (dyn DeviceDriver + Sync) ] {
    &self.drivers[..]
  }

  fn on_initialized(&self) {
    super::GPIO.map_pl011_uart();
    super::GPU.init_framebuffer();
  }
}

/// Device Drivers in order of init
static BSP_DRIVER_MAGANER: BSPDriverManager = BSPDriverManager {
  drivers: [
    &super::GPIO, 
    &super::UART,
    &super::MAILBOX,
    &super::GPU,
  ]
};

/// Return a reference to the driver manager.
pub fn manager() -> &'static impl driver::interface::DriverManager {
  &BSP_DRIVER_MAGANER
}