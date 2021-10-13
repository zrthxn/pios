use crate::sync::interface::Mutex;
use crate::{cpu, driver, sync::NullLock};
use crate::bsp::devices::common::MMIODerefWrapper;

use tock_registers::{
  interfaces::{Readable, Writeable},
  register_bitfields, register_structs,
  registers::{ReadOnly, ReadWrite, WriteOnly},
};

// Mailbox registers.
//
// Descriptions taken from https://jsandler18.github.io/extra/mailbox.html
register_bitfields! {
  u32,

  // The Read register is at offset 0x00 from the mailbox base, and facilitates reading messages from the GPU. 
  // The low 4 bits of the register denote which channel the message is from, 
  // and the high 28 bits are data. 
  READ [
    CHANNEL OFFSET(0) NUMBITS(4) [],
    DATA OFFSET(4) NUMBITS(28) [],
  ],

  // The Status register is at offset 0x18 from the mailbox base. 
  // Bit 30 of this register can tell you whether the Read register is empty and 
  // bit 31 can tell you whether the Write register is full. 
  STATUS [
    READ_EMPTY OFFSET(30) NUMBITS(1) [],
    WRITE_FULL OFFSET(31) NUMBITS(1) [],
  ],

  // The Write Register is at offset 0x20, and has similar form to the Read register.
  WRITE [
    CHANNEL OFFSET(0) NUMBITS(4) [],
    DATA OFFSET(4) NUMBITS(28) [],
  ],
}

register_structs! {
  #[allow(non_snake_case)]
  pub RegisterBlock {
    (0x00 => READ: ReadOnly<u32, READ::Register>),
    (0x18 => STATUS: ReadOnly<u32, STATUS::Register>),
    (0x20 => WRITE: WriteOnly<u32, WRITE::Register>),
    (0x24 => @END),
  }
}

type Registers = MMIODerefWrapper<RegisterBlock>;
pub struct MailboxInner {
  registers: Registers
}

impl MailboxInner {
  pub const unsafe fn new(mmio_start_addr: usize) -> Self {
    Self {
      registers: Registers::new(mmio_start_addr)
    }
  }

  fn read_channel(&mut self, c: u8) -> u32 {
    // Immediately zero out top 4 bits if any
    let channel = c & 0xF;

    // Spin until we have data for our channel and its not empty
    while 
      self.registers.READ.read(READ::CHANNEL) as u8 != channel &&
      self.registers.STATUS.matches_all(STATUS::READ_EMPTY::CLEAR)
    {
      cpu::nop();
    }

    self.registers.READ.read(READ::DATA) as u32
  }

  fn write_channel(&mut self, c: u8, d: u32) {
    // Immediately zero out top 4 bits if any
    let channel = c & 0x0F;
    // Immediately zero out bottom 4 bits if any, append channel
    let data = (d & 0xFFFFFFF0) | channel as u32;

    // Spin until we can write
    while self.registers.STATUS.matches_all(STATUS::WRITE_FULL::SET) {
      cpu::nop();
    }

    self.registers.WRITE.write(WRITE::DATA.val(data));
  }
}

pub struct MAILBOX {
  inner: NullLock<MailboxInner>
}

impl MAILBOX {
  pub const unsafe fn new(mmio_start_addr: usize) -> Self {
    Self {
      inner: NullLock::new(MailboxInner::new(mmio_start_addr))
    }
  }

  fn read_channel(&mut self, c: u8) -> u32 {
    self.inner.lock(|inner| inner.read_channel(c))
  }

  fn write_channel(&mut self, c: u8, d: u32) -> u32 {
    self.inner.lock(|inner| inner.write_channel(c, d))
  }
}

impl driver::interface::DeviceDriver for MAILBOX {
  fn compatible(&self) -> &'static str {
    "BCM MAILBOX"
  }
}