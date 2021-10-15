use core::ptr::write_volatile;

use crate::{bsp, driver, sync::{NullLock, interface::Mutex}, warn};
use bsp::raspi::video;

// https://jsandler18.github.io/tutorial/hdmi.html
// https://jsandler18.github.io/extra/prop-channel.html

pub struct GPUInner {
  framebuffer: u32,
  buffer_size: u32,
  px_size: u32,
  pitch: u32,
}

impl GPUInner {
  pub const unsafe fn new() -> Self {
    Self {
      framebuffer: 0,
      buffer_size: 0,
      px_size: 0,
      pitch: 0
    }
  }

  pub fn init_framebuffer(&mut self) {
    bsp::MAILBOX.set_screensize_msg([
      video::VIDEO::RESOLUTION::WIDTH as u32,
      video::VIDEO::RESOLUTION::HEIGHT as u32,
      video::VIDEO::COLOR_DEPTH as u32,
    ]);

    self.px_size = (video::VIDEO::COLOR_DEPTH / 8) as u32;
    self.pitch = (video::VIDEO::RESOLUTION::WIDTH as u32 * self.px_size);
    
    let (_buf, size) = bsp::MAILBOX.get_framebuffer_msg();
    self.framebuffer = _buf;
    self.buffer_size = size;
  }

  /// Clear screen 
  pub fn cls(&self) {
    const BLACK: u32 = 0x000000;
    self.write_color(BLACK)
  }

  /// Clear screen 
  pub fn white(&self) {
    const WHITE: u32 = 0xFFFFFF;
    self.write_color(WHITE)
  }

  pub fn write_color(&self, px: u32) {
    for y in 0..video::VIDEO::RESOLUTION::HEIGHT {
      for x in 0..video::VIDEO::RESOLUTION::WIDTH {
        unsafe { self.write_px(x, y, px) };
      }
    }
  }

  pub unsafe fn write_px(&self, x: u16, y: u16, pixel: u32) {
    let px = pixel & 0x00FFFFFF;
    write_volatile(
      (self.framebuffer + 
            (x as u32 * self.px_size)  + 
            (y as u32 * self.pitch) 
          ) as *mut u32, 
      px
    );
  }
}

pub struct GPU {
  inner: NullLock<GPUInner>
}

impl GPU {
  pub const unsafe fn new() -> Self {
    Self {
      inner: NullLock::new(GPUInner::new())
    }
  }


  pub fn init_framebuffer(&self) {
    self.inner.lock(|inner| inner.init_framebuffer())
  }

  pub fn cls(&self) {
    self.inner.lock(|inner| inner.cls())
  }
  
  pub fn white(&self) {
    self.inner.lock(|inner| inner.white())
  }

  pub unsafe fn write_color(&self, pixel: u32) {
    self.inner.lock(|inner| inner.write_color(pixel))
  }

  pub unsafe fn write_px(&self, x: u16, y: u16, pixel: u32) {
    self.inner.lock(|inner| inner.write_px(x, y, pixel))
  }
}

impl driver::interface::DeviceDriver for GPU {
  fn compatible(&self) -> &'static str {
    "BCM VideoCore"
  }
}