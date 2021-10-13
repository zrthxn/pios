/// Driver interfaces.
pub mod interface {
  /// Device Driver functions.
  pub trait DeviceDriver {
    /// Return a compatibility string for identifying the driver.
    fn compatible(&self) -> &'static str;

    /// Called by the kernel to bring up the device.
    /// During init, drivers might do stuff with system-wide impact.
    unsafe fn init(&self) -> Result<(), &'static str>{
      Ok(())
    }
  }

  /// Device driver management functions.
  ///
  /// The `BSP` is supposed to supply one global instance.
  pub trait DriverManager {
    /// Return a slice of references to all `BSP`-instantiated drivers,
    /// in the order of devices is the order in which `DeviceDriver::init()` is called.
    fn list_drivers(&self) -> &[ &'static (dyn DeviceDriver + Sync) ] {
      &[]
    }

    /// Initialization code that runs after driver init.
    /// For example, device driver code that depends on other drivers already being online.
    fn on_initialized(&self);
  }
}