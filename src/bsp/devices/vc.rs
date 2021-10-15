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
    bsp::MAILBOX.set_screensize_msg([
      bsp::raspi::video::VIDEO::RESOLUTION::WIDTH as u32,
      bsp::raspi::video::VIDEO::RESOLUTION::HEIGHT as u32,
      bsp::raspi::video::VIDEO::COLOR_DEPTH as u32,
    ])
  }
}

impl driver::interface::DeviceDriver for GPU {
  fn compatible(&self) -> &'static str {
    "BCM VideoCore"
  }
}