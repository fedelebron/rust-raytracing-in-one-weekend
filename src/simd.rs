#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use std::ops::{Div, Mul, Sub};

#[cfg(vec3a_sse2)]
use crate::Align16;

use std::mem;

type T = f32;

#[derive(Clone, Copy)]
#[repr(align(16), C)]
pub struct Vec3(pub(crate) __m128);

impl Vec3 {
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

impl Div<Vec3> for f32 {
  type Output = Vec3;
  #[inline]
  fn div(self, other: Vec3) -> Vec3 {
    unsafe { Vec3(_mm_div_ps(_mm_set1_ps(self), other.0)) }
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

type Point = Vec3;

pub struct Ray {
  pub origin: Point,
  pub direction: Vec3,
  pub time: T,
}

pub struct BoundingBox {
  minimum: Point,
  maximum: Point,
}

impl BoundingBox {
  pub fn hit(&self, mut t_min: T, mut t_max: T, ray: &Ray) -> bool {
    let inv_ds = 1.0 / ray.direction;
    let t0s = (self.minimum - ray.origin) * inv_ds;
    let t1s = (self.maximum - ray.origin) * inv_ds;
    let tsmaller = t0s.min(t1s);
    let tbigger = t0s.max(t1s);

    t_min = t_min.max(tsmaller.max_element());
    t_max = t_max.min(tbigger.min_element());

    return t_min < t_max;
  }
}
