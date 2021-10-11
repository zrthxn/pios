pub mod devices;

/// Conditional reexporting of Board Support Packages.
#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub mod raspi;
pub use raspi::*;

/// Conditional reexporting of Board Support Packages.
#[cfg(feature = "default")]
pub mod qemu;
pub use qemu::*;