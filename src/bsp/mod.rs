pub mod devices;
pub mod raspi;
pub mod serial;
pub mod qemu;

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;
