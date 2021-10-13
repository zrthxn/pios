use crate::{bsp, driver};

// https://jsandler18.github.io/tutorial/hdmi.html
// https://jsandler18.github.io/extra/prop-channel.html

pub struct GPU {
  framebuffer: u32
}

impl GPU {
  pub const unsafe fn new() -> Self {
    Self {
      framebuffer: 0
    }
  }

  pub fn init_framebuffer(&self) {
    bsp::MAILBOX.read_channel(1);
  }
}

impl driver::interface::DeviceDriver for GPU {
  fn compatible(&self) -> &'static str {
    "BCM VideoCore"
  }
}