// use core::alloc;
use core::alloc::Layout;
use core::mem::{size_of, size_of_val};
use core::ptr::{addr_of, read_volatile};

use crate::{print, println};
use crate::sync::interface::Mutex;
use crate::{cpu, driver, sync::NullLock};
use crate::bsp::devices::common::MMIODerefWrapper;

use tock_registers::interfaces::ReadWriteable;
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

  // READ_STATUS [
  //   // READ_EMPTY OFFSET(30) NUMBITS(1) [],
  //   STATUS [],
  // ],

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

  // WRITE_STATUS [
  //   // WRITE_FULL OFFSET(31) NUMBITS(1) [],
  //   WRITE_FULL [],
  // ],
  
  // WRITE_STATUS [
  //   READ_EMPTY OFFSET(30) NUMBITS(1) [],
  //   WRITE_FULL OFFSET(31) NUMBITS(1) [],
  // ],

}

register_structs! {
  #[allow(non_snake_case)]
  pub RegisterBlock {
    (0x00 => READ: ReadOnly<u32, READ::Register>),
    (0x18 => STATUS: ReadOnly<u32, STATUS::Register>),
    // (0x18 => READ_STATUS: ReadOnly<u32, READ_STATUS::Register>),
    (0x20 => WRITE: ReadWrite<u32, WRITE::Register>),
    // (0x38 => WRITE_STATUS: ReadOnly<u32, WRITE_STATUS::Register>),
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

  fn write_channel<T>(&self, c: u8, d: *const T) {
    // Immediately zero out top 4 bits if any
    let channel = (c & 0x0F) as u32;
    // Immediately zero out bottom 4 bits if any, append channel
    let data = ((d as u32) & 0x0FFFFFF0) | channel as u32;
    // let data = (d as u32) >> 4;
    println!("WRITE MBOX {:#x?}", data);
    // let data = (d as u32);

    // Spin until we can write
    while self.registers.STATUS.matches_all(STATUS::WRITE_FULL::SET) {
      cpu::nop();
    }

    self.registers.WRITE.set(data);
    // self.registers.WRITE.modify(WRITE::CHANNEL.val(channel));
    // self.registers.WRITE.modify(WRITE::DATA.val(data));
    // self.registers.WRITE.modify_no_read(self.registers.WRITE, WRITE::CHANNEL.val(channel));
    // self.registers.WRITE.modify_no_read(self.registers.WRITE, WRITE::DATA.val(data));
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
      0,
      TagType::SET_SCREEN_SIZE,     8, 0, values[0], values[1],
      TagType::SET_VIRTUAL_ADDRESS, 8, 0, values[0], values[1],
      TagType::SET_COLOR_DEPTH,     4, 0, values[2] * 3,
      TagType::NULL,
      0,0,0
    ]);

    println!("Sizeof MBM {}", size_of_val(&message));
    println!("Addrof MBM {:#x?}", addr_of!(message) as u32);

    self.write_channel(8, addr_of!(message));
    println!("Sent set_screensize_msg");
    
    // Just wait 10 cycles
    for _ in 0..10 {
      cpu::nop();
    }

    unsafe {
      // let response = self.read_channel(8) as *const MailboxMessage;

      println!("Rv READ {:#x?}", read_volatile((0x3F00_B880 + 0x0000_0000) as *const u32) as u32);
      println!("Rv STATUS {:#x?}", read_volatile((0x3F00_B880 + 0x0000_0018) as *const u32) as u32);
      println!("Rv WRITE {:#x?}", read_volatile((0x3F00_B880 + 0x0000_0020) as *const u32) as u32);

      println!("Response code {:x}", message[1]);
      println!("Rv Response code {}", read_volatile(
        (((addr_of!(message) as *const u32) as u32) + 4) as *const u32 
      ));
    }
  }

  pub fn get_framebuffer_msg(&self, values: [u64;1]) {

  }
}

impl driver::interface::DeviceDriver for MAILBOX {
  fn compatible(&self) -> &'static str {
    "BCM MAILBOX"
  }
}

#[allow(non_camel_case_types)]
mod TagType {
  pub const ALLOCATE_BUFFER    : u32 = 0x00004001;
  pub const RELEASE_BUFFER     : u32 = 0x00004801;

  pub const GET_SCREEN_SIZE    : u32 = 0x00004003;
  pub const GET_VIRTUAL_ADDRESS: u32 = 0x00004004;
  pub const GET_COLOR_DEPTH    : u32 = 0x00004005;
  pub const GET_BYTES_PER_ROW  : u32 = 0x00004008;

  pub const SET_SCREEN_SIZE    : u32 = 0x00004803;
  pub const SET_VIRTUAL_ADDRESS: u32 = 0x00004804;
  pub const SET_COLOR_DEPTH    : u32 = 0x00004805;

  pub const NULL               : u32 = 0x0000000;
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