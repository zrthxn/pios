use crate::{time, warn};
use core::time::Duration;
use cortex_a::{asm::barrier, registers::*};
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

const NS_PER_S: u64 = 1_000_000_000;

/// ARMv8 Generic Timer.
struct GenericTimer;

static TIME_MANAGER: GenericTimer = GenericTimer;

impl GenericTimer {
  #[inline(always)]
  fn read_cntpct(&self) -> u64 {
    // Prevent that the counter is read ahead of time due to out-of-order execution.
    unsafe { barrier::isb(barrier::SY) };
    CNTPCT_EL0.get()
  }
}

/// Return a reference to the time manager.
pub fn time_manager() -> &'static impl time::interface::TimeManager {
  &TIME_MANAGER
}

impl time::interface::TimeManager for GenericTimer {
  fn resolution(&self) -> Duration {
    Duration::from_nanos(NS_PER_S / (CNTFRQ_EL0.get() as u64))
  }

  fn uptime(&self) -> Duration {
    let current_count: u64 = self.read_cntpct() * NS_PER_S;
    let frq: u64 = CNTFRQ_EL0.get() as u64;

    Duration::from_nanos(current_count / frq)
  }

  fn spin_for(&self, duration: Duration) {
    // Instantly return on zero.
    if duration.as_nanos() == 0 {
      return;
    }

    // Calculate the register compare value.
    let frq = CNTFRQ_EL0.get();
    let x = match frq.checked_mul(duration.as_nanos() as u64) {
      None => {
        warn!("Spin duration too long, skipping");
        return;
      }
      Some(val) => val,
    };
    let tval = x / NS_PER_S;

    // Check if it is within supported bounds.
    let warn: Option<&str> = if tval == 0 {
      Some("smaller")
    // The upper 32 bits of CNTP_TVAL_EL0 are reserved.
    } else if tval > u32::max_value().into() {
      Some("bigger")
    } else {
      None
    };

    if let Some(w) = warn {
      warn!(
        "Spin duration {} than architecturally supported, skipping",
        w
      );
      return;
    }

    // Set the compare value register.
    CNTP_TVAL_EL0.set(tval);

    // Kick off the counting.                       // Disable timer interrupt.
    CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::SET);

    // ISTATUS will be '1' when cval ticks have passed. Busy-check it.
    while !CNTP_CTL_EL0.matches_all(CNTP_CTL_EL0::ISTATUS::SET) {}

    // Disable counting again.
    CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::CLEAR);
  }
}