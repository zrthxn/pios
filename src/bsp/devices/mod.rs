pub mod common;

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub mod gpio;
pub mod uart;
pub mod mailbox;
pub mod gpu;
