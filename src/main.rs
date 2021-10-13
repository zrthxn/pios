#![feature(asm)]
#![feature(global_asm)]
#![feature(trait_alias)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(const_fn_fn_ptr_basics)]

#![allow(clippy::upper_case_acronyms)]

#![no_std]
#![no_main]

mod bsp;
mod cpu;
mod driver;
mod panicinf;
mod console;
mod print;
mod sync;

/// Early init code. The Rust entry of the `kernel` binary.
/// The function is called from the assembly `_start` function.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order.
#[no_mangle]
pub unsafe fn _start_rust() -> ! {
  use crate::driver::interface::DriverManager;
  use crate::bsp::drivers::manager;

  for _driver in manager().list_drivers().iter() {
    if let Err(e) = _driver.init() {
      panic!("\t[x] Error loading driver: {}: {}", _driver.compatible(), e);
    }
  }
  
  manager().on_initialized();

  println!("[-] Device drivers loaded");
  println!("[-] Booting on [{}]\n", bsp::board_name());

  __main__()
}

/// Init Rust code
#[no_mangle]
fn __main__() -> ! {
  use crate::console::interface::Interactive;
  
  printpkg!();

  println!("\nHello World!\n");
  println!(
    "Characters Written: {:?}",
    bsp::serial().chars_written()
  );
  
  println!("[X] Kernel End");
  loop {

  }
}

