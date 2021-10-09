#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
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
pub unsafe fn _start_rust() {
  use crate::driver::interface::DriverManager;
  use crate::bsp::raspi::driver;

  println!("[-] Init Boot");

  for (i, _driver) in driver::manager().list_drivers().iter().enumerate() {
    print!("\t[-] Loading ({})", _driver.compatible());
    if let Err(e) = _driver.init() {
      panic!("\t[x] Error loading driver: {}: {}", _driver.compatible(), e);
    }
    print!("\t[DONE]\n");
  }

  println!("[-] Drivers Initialized\n");
  driver::manager().on_initialized();

  __init__()
}

/// Init Rust code
fn __init__() {
  use bsp::serial::serial;
  use console::interface::Interactive;
  use driver::interface::DriverManager;

  println!(
    "[0] {} version {}",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION")
  );
  println!("[1] Booting on: {}", bsp::raspi::board_name());
  println!("[2] Loading Drivers");

  let driverlist = bsp::raspi::driver::manager().list_drivers();
  for (i, _driver) in driverlist.iter().enumerate() {
    println!("\t[{}] {}", i + 1, _driver.compatible())
  }

  println!("\n[3] Hello World!\n");
  println!(
    "[4] Characters Written: {:?}",
    bsp::serial::serial().chars_written()
  );
  
  println!("[X] Kernel End");
}