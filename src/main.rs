#![no_std]
#![no_main]

mod bsp;
mod cpu;
mod panicinf;

/// The Rust entry of the `kernel` binary.
/// The function is called from the assembly `_start` function.
#[no_mangle]
pub unsafe fn _start_rust() {
  crate::__init__()
}

/// Init Rust code
unsafe fn __init__() {
  panic!()
}