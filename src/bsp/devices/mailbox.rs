use core::mem::{size_of, size_of_val};
use core::ptr::{addr_of, read_volatile, write_volatile};

use crate::{info, print, println, warn};
use crate::sync::interface::Mutex;
use crate::{cpu, driver, sync::NullLock};
use crate::bsp::devices::common::MMIODerefWrapper;

use tock_registers::{
  interfaces::{Readable, Writeable},
  register_bitfields, register_structs,
  registers::{ReadOnly, WriteOnly},
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
    CHANNEL OFFSET(0) NUMBITS(4),
    DATA OFFSET(4) NUMBITS(28),
  ],

  READ_STATUS [
    READ_EMPTY OFFSET(30) NUMBITS(1),
    READ_FULL OFFSET(31) NUMBITS(1),
  ],

  // The Status register is at offset 0x18 from the mailbox base. 
  // Bit 30 of this register can tell you whether the Read register is empty and 
  // bit 31 can tell you whether the Write register is full. 
  // STATUS [
  //   READ_EMPTY OFFSET(30) NUMBITS(1),
  //   WRITE_FULL OFFSET(31) NUMBITS(1),
  // ],

  // The Write Register is at offset 0x20, and has similar form to the Read register.
  WRITE [
    DATA OFFSET(0) NUMBITS(32),
  ],

  WRITE_STATUS [
    WRITE_EMPTY OFFSET(30) NUMBITS(1),
    WRITE_FULL OFFSET(31) NUMBITS(1),
  ],
}

register_structs! {
  #[allow(non_snake_case)]
  pub RegisterBlock {
    (0x00 => READ: ReadOnly<u32, READ::Register>),
    (0x18 => READ_STATUS: ReadOnly<u32, READ_STATUS::Register>),
    (0x20 => WRITE: WriteOnly<u32, WRITE::Register>),
    (0x38 => WRITE_STATUS: ReadOnly<u32, WRITE_STATUS::Register>),
    (0x41 => @END),
    // (0x24 => @END),
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

  /// ## !CAUTION!
  /// Calling this function IF there is no data to read, 
  /// it will cause the CPU to freeze. 
  fn read_channel(&self, c: u8) -> u32 {
    // Immediately zero out top 4 bits if any
    let channel = c & 0x0F;

    // If there is no data, return 0
    if self.registers.READ_STATUS.matches_all(READ_STATUS::READ_EMPTY::SET) {
      return 0 as u32;
    }

    // Spin until we have data for our channel and its not empty
    while self.registers.READ.read(READ::CHANNEL) as u8 != channel {
      cpu::nop();
    }

    unsafe{ read_volatile((0x3F00_B880) as *mut u32) as u32 }
    // For some reason these calls don't actually write anything?
    // self.registers.READ.read(READ::DATA) as u32
  }

  fn write_channel<T>(&self, c: u8, d: *const T) {
    // Immediately zero out top 4 bits if any
    let channel = (c & 0x0F) as u32;
    // Immediately zero out bottom 4 bits if any, append channel
    let data = ((d as u32) & 0xFFFF_FFF0) | channel;

    // Spin until we can write
    while self.registers.WRITE_STATUS.matches_all(WRITE_STATUS::WRITE_FULL::SET) {
      cpu::nop();
    }

    unsafe{ write_volatile((0x3F00_B880 + 0x20) as *mut _, data);}
    // For some reason these calls don't actually write anything?
    // self.registers.WRITE.write(WRITE::DATA.val(data));
  }
}

use aligned::{Aligned, A16};

pub struct MAILBOX {
  inner: NullLock<MailboxInner>
}

impl MAILBOX {
  pub const unsafe fn new(mmio_start_addr: usize) -> Self {
    Self {
      inner: NullLock::new(MailboxInner::new(mmio_start_addr))
    }
  }

  /// ## !CAUTION!
  /// Calling this function IF there is no data to read, 
  /// it will cause the CPU to freeze. 
  fn read_channel(&self, channel: u8) -> u32 {
    self.inner.lock(|inner| inner.read_channel(channel))
  }

  fn write_channel<T>(&self, channel: u8, data: *const T) {
    self.inner.lock(|inner| inner.write_channel(channel, data))
  }

  pub fn set_screensize_msg(&self, values: [u32;3]) {
    // let screen_size = (values[0] << 32) | values[1];
    // let message = MailboxMessage {
    //   buffer_size: 80,
    //   req_res_code: 0,
    //   tags: &[
    //     TagType::SET_SCREEN_SIZE,     8, 0, values[0], values[1],
    //     TagType::SET_VIRTUAL_ADDRESS, 8, 0, values[0], values[1],
    //     TagType::SET_COLOR_DEPTH,     4, 0, values[2] * 3,
    //   ],
    //   // tags: &[
    //   //   MessageTag::new(TagType::SET_SCREEN_SIZE,     screen_size),
    //   //   MessageTag::new(TagType::SET_VIRTUAL_ADDRESS, screen_size),
    //   //   MessageTag::new(TagType::SET_COLOR_DEPTH,     values[2] * 3),
    //   // ],
    //   null: TagType::NULL,
    //   padding: &[0,0,0]
    // };

    let message: Aligned<A16, [u32; 20]> = Aligned([
      80, 
      MessageCode::REQUEST,
      TagType::SET_SCREEN_SIZE,     8, MessageCode::REQUEST, values[0], values[1],
      TagType::SET_VIRTUAL_ADDRESS, 8, MessageCode::REQUEST, values[0], values[1],
      TagType::SET_COLOR_DEPTH,     4, MessageCode::REQUEST, values[2],
      TagType::NULL,
      0,0,0
    ]);

    self.write_channel(8, addr_of!(message));

    if message[1] == MessageCode::RESERROR {
      warn!("Error! Response code {:x}", message[1]);
    }
  }

  pub fn get_framebuffer_msg(&self) -> (u32,u32) {
    let message: Aligned<A16, [u32; 8]> = Aligned([
      32,
      MessageCode::REQUEST,
      TagType::ALLOCATE_BUFFER, 8, MessageCode::REQUEST, 16, 0,
      TagType::NULL
    ]);

    self.write_channel(8, addr_of!(message));
    info!("Assigned framebuffer at {:#x?} of {} bytes", message[5], message[6]);
    ( message[5], message[6] )
  }
}

impl driver::interface::DeviceDriver for MAILBOX {
  fn compatible(&self) -> &'static str {
    "BCM MAILBOX"
  }
}

#[allow(non_camel_case_types)]
mod TagType {
  pub const ALLOCATE_BUFFER    : u32 = 0x0004_0001;
  pub const RELEASE_BUFFER     : u32 = 0x0004_8001;

  pub const GET_SCREEN_SIZE    : u32 = 0x0004_0003;
  pub const GET_VIRTUAL_ADDRESS: u32 = 0x0004_0004;
  pub const GET_COLOR_DEPTH    : u32 = 0x0004_0005;
  pub const GET_BYTES_PER_ROW  : u32 = 0x0004_0008;

  pub const SET_SCREEN_SIZE    : u32 = 0x0004_8003;
  pub const SET_VIRTUAL_ADDRESS: u32 = 0x0004_8004;
  pub const SET_COLOR_DEPTH    : u32 = 0x0004_8005;

  pub const NULL               : u32 = 0x000_0000;
}

#[allow(non_camel_case_types)]
mod MessageCode {
  pub const REQUEST  : u32 = 0x0000_0000;
  pub const RESPONSE : u32 = 0x8000_0000;
  pub const RESERROR : u32 = 0x8000_0001;
}

// impl TagType {
//   fn size(&self) -> u8 {
//     match &self {
//       TagType::ALLOCATE_BUFFER     => 8,
//       TagType::GET_SCREEN_SIZE     => 8,
//       TagType::SET_SCREEN_SIZE     => 8,
//       TagType::GET_VIRTUAL_ADDRESS => 8,
//       TagType::SET_VIRTUAL_ADDRESS => 8,

//       TagType::GET_COLOR_DEPTH     => 4,
//       TagType::SET_COLOR_DEPTH     => 4,
//       TagType::GET_BYTES_PER_ROW   => 4,
//       TagType::RELEASE_BUFFER      => 0,
//       TagType::NULL                => 0,
//     }
//   }
// }

pub struct MessageTag {
  pub tag_type: u32,
  value_buffer_size: u32,
  pub req_res_code: u32,
  value_buffer: u64
}

impl MessageTag {
  fn new(tag_type: u32, value: u64) -> Self {
    Self {
      tag_type: tag_type as u32,
      value_buffer_size: 8,
      req_res_code: 0,
      value_buffer: value
    }
  }

  /// @TODO Add checking response code
  fn read(&self) -> Result<u64, ()> {
    Ok(self.value_buffer)
  }
}

#[repr(C, align(16))]
pub struct MailboxMessage<'msg> {
  buffer_size: u32,
  req_res_code: u32,
  // pub tags: &'msg[MessageTag],
  pub tags: &'msg[u32],
  null: u32,
  padding: &'msg[u32]
}