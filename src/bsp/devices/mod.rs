#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub mod common;
pub mod bcm2xxx;

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use bcm2xxx::*;