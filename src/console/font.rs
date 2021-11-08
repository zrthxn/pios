use crate::bsp::devices::gpu::Glyph;

pub const SPACE: Glyph = [[0;8];8];

pub mod uppercase {
  use crate::bsp::devices::gpu::Glyph;
  
  pub const A: Glyph = [[0;8];8];
  pub const B: Glyph = [[0;8];8];
  pub const C: Glyph = [[0;8];8];
  pub const D: Glyph = [[0;8];8];
  pub const E: Glyph = [
    [0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000],
    [0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000],
    [0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000],
    [0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF],
  ];
  pub const F: Glyph = [[0;8];8];
  pub const G: Glyph = [[0;8];8];
  pub const H: Glyph = [
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0xFFFFFF, 0xFFFFFF],
  ];
  pub const I: Glyph = [[0;8];8];
  pub const J: Glyph = [[0;8];8];
  pub const K: Glyph = [[0;8];8];
  pub const L: Glyph = [
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000, 0x000000],
    [0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF],
  ];
  pub const M: Glyph = [[0;8];8];
  pub const N: Glyph = [[0;8];8];
  pub const O: Glyph = [
    [0x000000, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0x000000],
    [0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0x000000, 0x000000, 0x000000, 0x000000, 0xFFFFFF, 0xFFFFFF],
    [0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF],
    [0x000000, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0x000000],
  ];
  pub const P: Glyph = [[0;8];8];
  pub const Q: Glyph = [[0;8];8];
  pub const R: Glyph = [[0;8];8];
  pub const S: Glyph = [[0;8];8];
  pub const T: Glyph = [[0;8];8];
  pub const U: Glyph = [[0;8];8];
  pub const V: Glyph = [[0;8];8];
  pub const W: Glyph = [[0;8];8];
  pub const X: Glyph = [[0;8];8];
  pub const Y: Glyph = [[0;8];8];
  pub const Z: Glyph = [[0;8];8];
}

#[allow(non_upper_case_globals)]
pub mod lowercase {
  use crate::bsp::devices::gpu::Glyph;
  
  pub const a: Glyph = [[0;8];8];
  pub const b: Glyph = [[0;8];8];
  pub const c: Glyph = [[0;8];8];
  pub const d: Glyph = [[0;8];8];
  pub const e: Glyph = [[0;8];8];
  pub const f: Glyph = [[0;8];8];
  pub const g: Glyph = [[0;8];8];
  pub const h: Glyph = [[0;8];8];
  pub const i: Glyph = [[0;8];8];
  pub const j: Glyph = [[0;8];8];
  pub const k: Glyph = [[0;8];8];
  pub const l: Glyph = [[0;8];8];
  pub const m: Glyph = [[0;8];8];
  pub const n: Glyph = [[0;8];8];
  pub const o: Glyph = [[0;8];8];
  pub const p: Glyph = [[0;8];8];
  pub const q: Glyph = [[0;8];8];
  pub const r: Glyph = [[0;8];8];
  pub const s: Glyph = [[0;8];8];
  pub const t: Glyph = [[0;8];8];
  pub const u: Glyph = [[0;8];8];
  pub const v: Glyph = [[0;8];8];
  pub const w: Glyph = [[0;8];8];
  pub const x: Glyph = [[0;8];8];
  pub const y: Glyph = [[0;8];8];
  pub const z: Glyph = [[0;8];8];
}

pub mod numbers {
  use crate::bsp::devices::gpu::Glyph;

  pub const ZERO: Glyph = [[0;8];8];
  pub const ONE: Glyph = [[0;8];8];
  pub const TWO: Glyph = [[0;8];8];
  pub const THREE: Glyph = [[0;8];8];
  pub const FOUR: Glyph = [[0;8];8];
  pub const FIVE: Glyph = [[0;8];8];
  pub const SIX: Glyph = [[0;8];8];
  pub const SEVEN: Glyph = [[0;8];8];
  pub const EIGHT: Glyph = [[0;8];8];
  pub const NINE: Glyph = [[0;8];8];
}

pub mod symbols {
  
}


pub fn bitmap(ch: char) -> Glyph {
  match ch {
    'A' => uppercase::A,
    'B' => uppercase::B,
    'C' => uppercase::C,
    'D' => uppercase::D,
    'E' => uppercase::E,
    'F' => uppercase::F,
    'G' => uppercase::G,
    'H' => uppercase::H,
    'I' => uppercase::I,
    'J' => uppercase::J,
    'K' => uppercase::K,
    'L' => uppercase::L,
    'M' => uppercase::M,
    'N' => uppercase::N,
    'O' => uppercase::O,
    'P' => uppercase::P,
    'Q' => uppercase::Q,
    'R' => uppercase::R,
    'S' => uppercase::S,
    'T' => uppercase::T,
    'U' => uppercase::U,
    'V' => uppercase::V,
    'W' => uppercase::W,
    'X' => uppercase::X,
    'Y' => uppercase::Y,
    'Z' => uppercase::Z,

    'a' => lowercase::a,
    'b' => lowercase::b,
    'c' => lowercase::c,
    'd' => lowercase::d,
    'e' => lowercase::e,
    'f' => lowercase::f,
    'g' => lowercase::g,
    'h' => lowercase::h,
    'i' => lowercase::i,
    'j' => lowercase::j,
    'k' => lowercase::k,
    'l' => lowercase::l,
    'm' => lowercase::m,
    'n' => lowercase::n,
    'o' => lowercase::o,
    'p' => lowercase::p,
    'q' => lowercase::q,
    'r' => lowercase::r,
    's' => lowercase::s,
    't' => lowercase::t,
    'u' => lowercase::u,
    'v' => lowercase::v,
    'w' => lowercase::w,
    'x' => lowercase::x,
    'y' => lowercase::y,
    'z' => lowercase::z,

    _ => SPACE
  }
}