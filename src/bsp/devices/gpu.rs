#![allow(dead_code)]

use core::ptr::write_volatile;

use crate::{bsp, driver, sync::{NullLock, interface::Mutex}, warn};
use bsp::raspi::video::{VIDEO, VIDEO::RESOLUTION};

// https://jsandler18.github.io/tutorial/hdmi.html
// https://jsandler18.github.io/extra/prop-channel.html

pub type Frame = [[u32; RESOLUTION::WIDTH as usize]; RESOLUTION::HEIGHT as usize];
pub type Field = [[u32; 16]; 16];
pub type Glyph = [[u32;  8];  8];

pub struct GPUInner {
  framebuffer: u32,
  buffer_size: u32,
  px_size: u32,
  pitch: u32,
  width: u16,
  height: u16,
  color_depth: u8
}

impl GPUInner {
  pub const unsafe fn new() -> Self {
    Self {
      framebuffer: 0,
      buffer_size: 0,
      px_size: 0,
      pitch: 0,
      width: RESOLUTION::WIDTH,
      height: RESOLUTION::HEIGHT,
      color_depth: VIDEO::COLOR_DEPTH,
    }
  }

  pub fn init_framebuffer(&mut self) {
    bsp::MAILBOX.set_screensize_msg([
      self.width as u32,
      self.height as u32, 
      self.color_depth as u32
    ]);

    self.px_size = (self.color_depth * 3 / 8) as u32;
    self.pitch = self.width as u32 * self.px_size;
    
    let (_buf, size) = bsp::MAILBOX.get_framebuffer_msg();
    self.framebuffer = _buf;
    self.buffer_size = size;

    if size < ((self.height * self.width) as u32 * self.px_size).into() {
      warn!("Allocated framebuffer is smaller than screen size");
    }
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

  pub fn write_glyph(&self, loc: (u16,u16), gl: Glyph) {
    let (_x, _y) = loc;
    for y in 0..8 {
      for x in 0..8 {
        unsafe { self.write_px(_x + x, _y + y, gl[y as usize][x as usize]) };
      }
    }
  }
  
  pub fn write_field(&self, loc: (u16,u16), fd: Field) {
    let (_x, _y) = loc;
    for y in 0..16 {
      for x in 0..16 {
        unsafe { self.write_px(_x + x, _y + y, fd[y as usize][x as usize]) };
      }
    }
  }

  pub fn write_frame(&self, fr: Frame) {
    for y in 0..self.height {
      for x in 0..self.width {
        unsafe { self.write_px(x , y, fr[y as usize][x as usize]) };
      }
    }
  }

  pub fn write_color(&self, px: u32) {
    for y in 0..self.height {
      for x in 0..self.width {
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

  pub fn write_glyph(&self, loc: (u16,u16), glyph: Glyph) {
    self.inner.lock(|inner| inner.write_glyph(loc, glyph))
  }
  
  pub fn write_field(&self, loc: (u16,u16), fd: Field) {
    self.inner.lock(|inner| inner.write_field(loc, fd))
  }

  pub fn write_frame(&self, fr: Frame) {
    self.inner.lock(|inner| inner.write_frame(fr))
  }

  pub fn write_color(&self, px: u32) {
    self.inner.lock(|inner| inner.write_color(px))
  }
}

impl driver::interface::DeviceDriver for GPU {
  fn compatible(&self) -> &'static str {
    "BCM VideoCore"
  }
}