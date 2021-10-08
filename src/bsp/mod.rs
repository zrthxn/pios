pub mod console;

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;
