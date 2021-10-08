#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(trait_alias)]

#![no_std]
#![no_main]

mod bsp;
mod cpu;
mod panicinf;
mod console;
mod print;
mod sync;

/// The Rust entry of the `kernel` binary.
/// The function is called from the assembly `_start` function.
#[no_mangle]
pub unsafe fn _start_rust() {
  crate::__init__()
}

/// Init Rust code
unsafe fn __init__() {
  use crate::console::interface::History;

  println!("[0] Hello World!");
  println!(
    "[1] Characters Written: {:?}",
    bsp::console::console().chars_written()
  );
  
  println!("[2] Kernel End");
  cpu::halt()
}