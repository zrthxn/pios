//! Common device driver code.

use core::{marker::PhantomData, ops};

pub struct MMIODerefWrapper<T> {
  start_addr: usize,
  phantom: PhantomData<fn() -> T>,
}

impl<T> MMIODerefWrapper<T> {
  pub const unsafe fn new(_addr: usize) -> Self {
    Self {
      start_addr: _addr,
      phantom: PhantomData,
    }
  }
}

impl<T> ops::Deref for MMIODerefWrapper<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe {
      &*(self.start_addr as *const _)
    }
  }
}