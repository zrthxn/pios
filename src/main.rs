#![feature(trait_alias)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(const_fn_fn_ptr_basics)]

#![allow(clippy::upper_case_acronyms)]

#![no_std]
#![no_main]

mod bsp;
mod arch;
mod driver;
mod panicinf;
mod console;
mod print;
mod sync;
mod time;
mod exception;

/// Early init code. The Rust entry of the `kernel` binary.
/// The function is called from the assembly `_start` function.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order.
#[no_mangle]
pub unsafe fn __init__() -> ! {
  use crate::driver::interface::DriverManager;
  use crate::bsp::drivers::manager;

  println!("[-] Init boot sequence");
  println!("[-] Booting on [{}]", bsp::board_name());
  println!();

  for _driver in manager().list_drivers().iter() {
    if let Err(e) = _driver.init() {
      panic!("\t[x] Error loading driver: {}: {}", _driver.compatible(), e);
    }
  }

  manager().on_initialized();

  info!("Device drivers loaded");
  for _driver in manager().list_drivers().iter() {
    info!("[-] {}", _driver.compatible());
  }

  let (_, privilege_level) = arch::exception::current_privilege_level();
  info!("Current privilege level [{}]", privilege_level);

  info!("Exception handling state");
  arch::exception::asynchronous::print_state();

  use time::interface::TimeManager;
  let time_res = arch::time::time_manager().resolution();
  info!("Architectural timer resolution: {} ns", time_res.as_nanos());

  println!();
  
  __main__()
}

/// Init Rust code
#[no_mangle]
fn __main__() -> ! {
  use crate::console::CONSOLE;
  
  printpkg!();

  println!("\nHello World!\n");
  clsc!();

  CONSOLE.write_str("HELLO\n");
  CONSOLE.write_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ\n");
  CONSOLE.write_str("abcdefghijklmnopqrstuvwxyz\n");
  CONSOLE.write_str("0123456789\n");

  let (s, _s) = CONSOLE.read_str();
  for i in 0..s {
    print!("{}", _s[i])
  }
  print!("\n");

  println!("[X] Kernel End");
  loop {}
}

