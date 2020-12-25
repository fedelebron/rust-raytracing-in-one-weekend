extern crate image;
use crate::vec3::*;
use image::{ImageBuffer, RgbImage};

pub struct Canvas {
  img: RgbImage,
}

type T = f32;

fn clamp(x: T, lo: T, hi: T) -> T {
  if x < lo {
    return lo;
  }
  if x > hi {
    return hi;
  }
  return x;
}

impl Canvas {
  pub fn new(width: u32, height: u32) -> Canvas {
    Canvas {
      img: ImageBuffer::new(width, height),
    }
  }

  pub fn draw(&mut self, x: u32, y: u32, c: &Color) {
    // We flip the y coordinate because conceptually our origin is the bottom-left corner.
    self
      .img
      .put_pixel(x, self.img.height() - 1 - y, Self::to_rgb(c))
  }

  pub fn save(&self, name: &str) {
    self.img.save(name).unwrap();
  }

  fn to_rgb(c: &Color) -> image::Rgb<u8> {
    let r = 256.0 * clamp(c.r().sqrt(), 0.0, 0.999);
    let g = 256.0 * clamp(c.g().sqrt(), 0.0, 0.999);
    let b = 256.0 * clamp(c.b().sqrt(), 0.0, 0.999);
    image::Rgb([r.floor() as u8, g.floor() as u8, b.floor() as u8])
  }
}
