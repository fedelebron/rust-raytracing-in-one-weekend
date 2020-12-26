extern crate image;
use image::{RgbImage, Rgb, open};
use std::sync::Arc;
use crate::vec3::*;

type T = f32;

#[derive(Clone)]
pub enum Texture {
  Color(Color),
  Image(Arc<RgbImage>),
  Checkers(Arc<Texture>, Arc<Texture>)
}

impl Texture {
  // u, v are surface coordinates.
  pub fn value(&self, u: T, v: T, p: Point) -> Color {
    match self {
      Texture::Color(x) => *x,
      Texture::Checkers(ref white, ref black) => get_checkers_color(&**white, &**black, u, v, p),
      Texture::Image(ref buf) => get_image_color(&**buf, u, v, p)
    }
  }

  pub fn from_image_filename(filename: &str) -> Texture {
    Texture::Image(Arc::new(open(filename).unwrap().into_rgb8()))
  }
}

fn get_checkers_color(white: &Texture, black: &Texture, u: T, v: T, p: Point) -> Color {
  let w = 10.0 * p.0;
  let sines = w.x().sin() * w.y().sin() * w.z().sin();
  if sines < 0.0 {
    white.value(u, v, p)
  } else {
    black.value(u, v, p)
  }
}

fn clamp<T: PartialOrd>(x: T, lo: T, hi: T) -> T {
  if x < lo {
    return lo;
  }
  if x > hi {
    return hi;
  }
  return x;
}

fn get_image_color(image: &RgbImage, mut u: T, mut v: T, _p: Point) -> Color {
  u = clamp(u, 0.0, 1.0);
  v = 1.0 - clamp(v, 0.0, 1.0);

  let i = clamp((u * image.width() as f32) as u32, 0, image.width() - 1);
  let j = clamp((v * image.height() as f32) as u32, 0, image.height() - 1);

  let color_scale = 1.0 / 255.0;
  let pixel:&Rgb<u8> = image.get_pixel(i, j);

  color_scale * Color::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32) 
}
