mod nulllock;

pub use nulllock::*;

/// Any object implementing this trait guarantees exclusive access to the data wrapped within
/// the Mutex for the duration of the provided closure.
pub mod interface {
  pub trait Mutex {
    type Data;
    /// Takes closure (callback) as arg, which is given a &mut of
    /// our `Data` and returns type `R`, which this fn also returns.
    fn lock<R>(&self, cb: impl FnOnce(&mut Self::Data) -> R) -> R;
  }
}
