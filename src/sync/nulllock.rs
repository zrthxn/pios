use core::cell::UnsafeCell;
use super::interface;

/// A pseudo-lock.
///
/// The lock will only be used as long as it is safe to do so, i.e. as long as the kernel is
/// executing single-threaded, aka only running on a single core with interrupts disabled.
pub struct NullLock<T> where T: ?Sized, {
  data: UnsafeCell<T>,
}

unsafe impl<T> Send for NullLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for NullLock<T> where T: ?Sized + Send {}

impl<T> NullLock<T> {
  /// Create an instance.
  pub const fn new(data: T) -> Self {
    Self {
      data: UnsafeCell::new(data),
    }
  }
}

/// In contrast to a real Mutex implementation, does not actually protect against concurrent access from
/// other cores to the contained data. This part is preserved for later lessons.
impl<T> interface::Mutex for NullLock<T> {
  type Data = T;

  fn lock<R>(&self, cb: impl FnOnce(&mut Self::Data) -> R) -> R {
    // In a real lock, there would be code encapsulating this line that ensures that this
    // mutable reference will ever only be given out once at a time.
    let data = unsafe {
      &mut *self.data.get()
    };

    cb(data)
  }
}
