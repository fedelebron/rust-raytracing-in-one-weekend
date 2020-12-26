use std::f32::consts::PI;
use rand::Rng;

use std::ops::{Add, AddAssign, DivAssign, Div, Mul, MulAssign, Sub, SubAssign, Neg};

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

type T = f32;

pub const fn _mm_shuffle(z: u32, y: u32, x: u32, w: u32) -> i32 {
    ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

#[derive(Clone, Copy, Debug)]
#[repr(align(16), C)]
pub struct Vec3(pub(crate) __m128);
impl Vec3 {
  pub fn new(x: T, y: T, z: T) -> Self {
    unsafe {
      Self(_mm_set_ps(0.0, z, y, x))
    }
  }
  pub fn x(self) -> T {
    unsafe {
      _mm_cvtss_f32(self.0)
    }
  }
  pub fn y(self) -> T {
    unsafe {
      _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, _mm_shuffle(0, 0, 0, 1)))
    }
  }
  pub fn z(self) -> T {
    unsafe {
      _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, _mm_shuffle(0, 0, 0, 2)))
    }
  }
  pub fn random_unit() -> Vec3 {
    let mut rng = rand::thread_rng();
    let theta = (2.0 * PI * rng.gen::<T>()) as T;
    let phi = ((1.0 - 2.0 * rng.gen::<T>()) as T).acos();
    Vec3::new(
      phi.sin() * theta.cos(),
      phi.sin() * theta.sin(),
      phi.cos(),
    )
  }
  pub fn near_zero(self) -> bool {
    return self.abs().max_element() <= 1e-8;
  }
  pub fn normalize(self) -> Self {
    unsafe {
      let dot = self.dot_as_vec3(self);
      Self(_mm_div_ps(self.0, _mm_sqrt_ps(dot.0)))
    }
  }
  pub fn cross(self, rhs: Self) -> Self {
    unsafe {
      let lhszxy = _mm_shuffle_ps(self.0, self.0, 0b01_01_00_10);
      let rhszxy = _mm_shuffle_ps(rhs.0, rhs.0, 0b01_01_00_10);
      let lhszxy_rhs = _mm_mul_ps(lhszxy, rhs.0);
      let rhszxy_lhs = _mm_mul_ps(rhszxy, self.0);
      let sub = _mm_sub_ps(lhszxy_rhs, rhszxy_lhs);
      Self(_mm_shuffle_ps(sub, sub, 0b01_01_00_10))
    }
  }
  pub fn dot(self, rhs: Self) -> T {
    unsafe {
      _mm_cvtss_f32(self.dot_as_m128(rhs))
    }
  }
  fn dot_as_m128(self, rhs: Self) -> __m128 {
    unsafe {
      let x2_y2_z2_w2 = _mm_mul_ps(self.0, rhs.0);
      let y2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_01);
      let z2_0_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_00_10);
      let x2y2_0_0_0 = _mm_add_ss(x2_y2_z2_w2, y2_0_0_0);
      _mm_add_ss(x2y2_0_0_0, z2_0_0_0)
    }
  }
  pub fn dot_as_vec3(self, rhs: Self) -> Self {
    unsafe {
      let dot_in_x = self.dot_as_m128(rhs);
      Vec3(_mm_shuffle_ps(dot_in_x, dot_in_x, 0b00_00_00_00))
    }
  }
  pub fn abs(&self) -> Self {
    unsafe {
      Self(_mm_and_ps(self.0,
                      _mm_castsi128_ps(_mm_set1_epi32(0x7f_ff_ff_ff)),
      ))
    }
  }
  pub fn norm_squared(self) -> T {
    self.dot(self)
  }
  pub fn norm(self) -> T {
    self.norm_squared().sqrt()
  }
  #[inline]
  pub fn min(self, other: Self) -> Self {
    unsafe { Self(_mm_min_ps(self.0, other.0)) }
  }

  #[inline]
  pub fn max(self, other: Self) -> Self {
    unsafe { Self(_mm_max_ps(self.0, other.0)) }
  }
  #[inline]
  pub fn min_element(self) -> f32 {
    unsafe {
      let v = self.0;
      let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b01_01_10_10));
      let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
      _mm_cvtss_f32(v)
    }
  }

  #[inline]
  pub fn max_element(self) -> f32 {
    unsafe {
      let v = self.0;
      let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_10_10));
      let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
      _mm_cvtss_f32(v)
    }
  }
}

impl Neg for Vec3 {
  type Output = Vec3;
  fn neg(self) -> Self {
    (-1.0) * self
  }
}

impl Add for Vec3 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    unsafe {
      Vec3(_mm_add_ps(self.0, rhs.0))
    }
  }
}

impl AddAssign<Vec3> for Vec3 {
  fn add_assign(&mut self, rhs: Vec3) {
    *self = (*self) + rhs;
  }
}


impl Div<Vec3> for T {
  type Output = Vec3;
  #[inline]
  fn div(self, other: Vec3) -> Vec3 {
    unsafe { Vec3(_mm_div_ps(_mm_set1_ps(self), other.0)) }
  }
}

impl DivAssign<T> for Vec3 {
  fn div_assign(&mut self, rhs: T) {
    self.0 = (*self / rhs).0
  }
}
impl Div<f32> for Vec3 {
  type Output = Vec3;
  #[inline]
  fn div(self, other: T) -> Vec3 {
    unsafe { Vec3(_mm_div_ps(self.0, _mm_set1_ps(other))) }
  }
}

impl Sub for Vec3 {
  type Output = Self;
  #[inline]
  fn sub(self, other: Self) -> Self {
    unsafe { Self(_mm_sub_ps(self.0, other.0)) }
  }
}

impl Mul<Vec3> for Vec3 {
  type Output = Self;
  #[inline]
  fn mul(self, other: Self) -> Self {
    unsafe { Self(_mm_mul_ps(self.0, other.0)) }
  }
}

impl Mul<T> for Vec3 {
  type Output = Self;
  #[inline]
  fn mul(self, other: f32) -> Self {
    unsafe { Self(_mm_mul_ps(self.0, _mm_set1_ps(other))) }
  }
}

impl Mul<Vec3> for T {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Vec3 {
    rhs * self
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Point(pub Vec3);

impl Point {
  pub fn new(x: T, y: T, z: T) -> Point {
    Point(Vec3::new(x, y, z))
  }
}

impl Sub<Vec3> for Point {
  type Output = Self;
  fn sub(self, rhs: Vec3) -> Point {
    Point(self.0 - rhs)
  }
}

impl Sub<Point> for Point {
  type Output = Vec3;
  fn sub(self, rhs: Point) -> Vec3 {
    self.0 - rhs.0
  }
}

impl Add<Vec3> for Point {
  type Output = Self;
  fn add(self, rhs: Vec3) -> Point {
    Point(self.0 + rhs)
  }
}

impl Add<Point> for Point {
  type Output = Self;
  fn add(self, rhs: Point) -> Point {
    Point(self.0 + rhs.0)
  }
}

impl Mul<T> for Point {
  type Output = Self;
  fn mul(self, rhs: T) -> Self {
    Point(rhs * self.0)
  }
}

impl Mul<Point> for T {
  type Output = Point;
  fn mul(self, rhs: Point) -> Point {
    rhs * self
  }
}

#[derive(Clone, Copy)]
pub struct Color(pub Vec3);
impl Color {
  pub fn new(x: T, y: T, z: T) -> Color {
    Color(Vec3::new(x, y, z))
  }

  pub fn random() -> Color {
    let mut rng = rand::thread_rng();
    Color::new(rng.gen(), rng.gen(), rng.gen())
  }

  pub fn random_range(lo: T, hi: T) -> Color {
    let mut rng = rand::thread_rng();
    Color::new(
      rng.gen_range(lo..hi),
      rng.gen_range(lo..hi),
      rng.gen_range(lo..hi),
    )
  }

  pub fn r(self) -> T {
    self.0.x()
  }
  pub fn g(self) -> T {
    self.0.y()
  }
  pub fn b(self) -> T {
    self.0.z()
  }
}

impl Mul<T> for Color {
  type Output = Self;
  fn mul(self, rhs: T) -> Color {
    Color(self.0 * rhs)
  }
}

impl Mul<Color> for T {
  type Output = Color;
  fn mul(self, rhs: Color) -> Color {
    Color(rhs.0 * self)
  }
}

impl Mul<Color> for Color {
  type Output = Color;
  fn mul(self, rhs: Color) -> Color {
    Color(self.0 * rhs.0)
  }
}

impl Add<Color> for Color {
  type Output = Color;
  fn add(self, rhs: Color) -> Color {
    Color(self.0 + rhs.0)
  }
}

impl AddAssign<Color> for Color {
  fn add_assign(&mut self, rhs: Color) {
    self.0 += rhs.0;
  }
}

impl DivAssign<T> for Color {
  fn div_assign(&mut self, rhs: T) {
    self.0 /= rhs;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::vec3;
  #[test]
  fn test_vec() {
    let v = vec3::Vec3::new(1.0, 2.0, 3.0);
    let w = vec3::Vec3::new(7.0, 2.0, -4.0);
    let v2 = Vec3::new(1.0, 2.0, 3.0);
    let w2 = Vec3::new(7.0, 2.0, -4.0);

    let u = v.cross(w);
    let u2 = v2.cross(w2);
    assert_eq!(u.x(), u2.x());
    assert_eq!(u.y(), u2.y());
    assert_eq!(u.z(), u2.z());
  }
  #[test]
  fn test_scalar() {
    let v = vec3::Vec3::new(1.0, 2.0, 3.0);
    let w = vec3::Vec3::new(7.0, 2.0, -4.0);
    let v2 = Vec3::new(1.0, 2.0, 3.0);
    let w2 = Vec3::new(7.0, 2.0, -4.0);

    let u = v.dot(w);
    let u2 = v2.dot(w2);
    assert_eq!(u, u2);
  }
  #[test]
  fn test_unary_scalar() {
    let v = vec3::Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(1.0, 2.0, 3.0);

    let u = v.norm();
    let u2 = v2.norm();
    assert_eq!(u, u2);
  }
  #[test]
  fn test_unary_vec() {
    let v = vec3::Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(1.0, 2.0, 3.0);

    let u = v.normalize();
    let u2 = v2.normalize();
    assert_eq!(u.x(), u2.x());
    assert_eq!(u.y(), u2.y());
    assert_eq!(u.z(), u2.z());
  }

}
