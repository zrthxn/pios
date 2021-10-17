use cortex_a::registers::*;
use tock_registers::interfaces::Readable;

use crate::exception::PrivilegeLevel;

/// The processing element's current privilege level.
pub fn current_privilege_level() -> (PrivilegeLevel, &'static str) {
  let el = CurrentEL.read_as_enum(CurrentEL::EL);
  match el {
    Some(CurrentEL::EL::Value::EL2) => (PrivilegeLevel::Hypervisor, "EL2"),
    Some(CurrentEL::EL::Value::EL1) => (PrivilegeLevel::Kernel, "EL1"),
    Some(CurrentEL::EL::Value::EL0) => (PrivilegeLevel::User, "EL0"),
    _ => (PrivilegeLevel::Unknown, "Unknown"),
  }
}

pub mod asynchronous {  
  use cortex_a::registers::*;
  use tock_registers::interfaces::Readable;

  trait DaifField {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register>;
  }

  struct Debug;
  struct SError;
  struct IRQ;
  struct FIQ;

  impl DaifField for Debug {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
      DAIF::D
    }
  }

  impl DaifField for SError {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
      DAIF::A
    }
  }

  impl DaifField for IRQ {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
      DAIF::I
    }
  }

  impl DaifField for FIQ {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
      DAIF::F
    }
  }

  fn is_masked<T>() -> bool
  where
    T: DaifField,
  {
    DAIF.is_set(T::daif_field())
  }

  /// Print the AArch64 exceptions status.
  #[rustfmt::skip]
  pub fn print_state() {
    use crate::info;

    let to_mask_str = |x| -> _ {
      if x { "Masked" } else { "Unmasked" }
    };

    info!("      Debug:  {}", to_mask_str(is_masked::<Debug>()));
    info!("      SError: {}", to_mask_str(is_masked::<SError>()));
    info!("      IRQ:    {}", to_mask_str(is_masked::<IRQ>()));
    info!("      FIQ:    {}", to_mask_str(is_masked::<FIQ>()));
  }
}