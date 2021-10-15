pub use crate::cpu::exception::current_privilege_level;

/// Kernel privilege levels.
#[allow(missing_docs)]
#[derive(PartialEq)]
pub enum PrivilegeLevel {
  User,
  Kernel,
  Hypervisor,
  Unknown,
}