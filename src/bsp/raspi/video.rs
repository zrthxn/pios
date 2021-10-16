// Properties for video out
// Access function for device
// Font renderring/Rendering Characters

#[rustfmt::skip]
#[allow(non_snake_case)]
pub mod VIDEO {
  /// Bits of color per pixel
  pub const COLOR_DEPTH: u8 = 8;

  pub mod RESOLUTION {
    pub const HEIGHT: u16 = 480;
    pub const WIDTH:  u16 = 640;
  }
}