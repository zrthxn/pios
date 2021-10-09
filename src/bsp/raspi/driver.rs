use crate::driver;
use driver::interface::DeviceDriver;

struct BSPDriverManager {
  drivers: [ &'static (dyn DeviceDriver + Sync); 2 ]
}

impl driver::interface::DriverManager for BSPDriverManager {
  fn list_drivers(&self) -> &[ &'static (dyn DeviceDriver + Sync) ] {
    &self.drivers[..]
  }

  fn on_initialized(&self) {
    super::GPIO.map_pl011_uart();
  }
}

/// Device Driver Manager type.
static BSP_DRIVER_MAGANER: BSPDriverManager = BSPDriverManager {
  drivers: [&super::GPIO, &super::UART]
};

/// Return a reference to the driver manager.
pub fn manager() -> &'static impl driver::interface::DriverManager {
  &BSP_DRIVER_MAGANER
}