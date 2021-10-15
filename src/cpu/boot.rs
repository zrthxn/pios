use cortex_a::{asm, registers::*};
use tock_registers::interfaces::Writeable;

#[no_mangle]
pub unsafe extern "C" fn _start_rust(__boot_core_stack_end_exclusive: u64) -> ! {
  // Prepare EL2 to EL1 transition
  // Enable timer counter registers for EL1.
  CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

  // No offset for reading the counters.
  CNTVOFF_EL2.set(0);

  // Set EL1 execution state to AArch64.
  HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

  // Set up a simulated exception return.
  //
  // First, fake a saved program status where all interrupts were masked and SP_EL1 was used as a
  // stack pointer.
  SPSR_EL2.write(
    SPSR_EL2::D::Masked
      + SPSR_EL2::A::Masked
      + SPSR_EL2::I::Masked
      + SPSR_EL2::F::Masked
      + SPSR_EL2::M::EL1h,
  );

  // Second, let the link register point to __init__().
  ELR_EL2.set(crate::__init__ as *const () as u64);

  // Set up SP_EL1 (stack pointer), which will be used by EL1 once we "return" to it. Since there
  // are no plans to ever return to EL2, just re-use the same stack.
  SP_EL1.set(__boot_core_stack_end_exclusive);

  // Use `eret` to "return" to EL1. This results in execution of __init__() in EL1.
  asm::eret()
}