use crate::ray::*;
use crate::vec3::*;
use std::cmp::Ordering;
use std::mem;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

type T = f32;

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
  minimum: Point,
  maximum: Point,
}

impl BoundingBox {
  pub fn new(minimum: Point, maximum: Point) -> BoundingBox {
    BoundingBox { minimum, maximum }
  }
  pub fn hit(&self, mut t_min: T, mut t_max: T, ray: &Ray) -> bool {
    return self.hit_vectorized(t_min, t_max, ray);
    /*for a in 0..3 {
      let inv_d = 1.0 / ray.direction[a];
      let mut t0 = (self.minimum[a] - ray.origin[a]) * inv_d;
      let mut t1 = (self.maximum[a] - ray.origin[a]) * inv_d;
      if inv_d < 1.0 {
        mem::swap(&mut t0, &mut t1);
      }
      t_min = t0.max(t_min); // if t0 > t_min { t0 } else { t_min };
      t_max = t1.min(t_max); // = if t1 < t_max { t1 } else { t_max };
      if t_max <= t_min {
        return false;
      }
    }
    return true;*/
  }

  fn hit_vectorized(&self, mut t_min: T, mut t_max: T, ray: &Ray) -> bool {
    let inv_ds = 1.0 / ray.direction;
    let t0s = (self.minimum - ray.origin) * inv_ds;
    let t1s = (self.maximum - ray.origin) * inv_ds;
    let tsmaller = t0s.min(t1s);
    let tbigger = t0s.max(t1s);

    t_min = t_min.max(tsmaller.max_element());
    t_max = t_max.min(tbigger.min_element());

    return t_min < t_max;
  }

  pub fn surrounding_box(bb1: &BoundingBox, bb2: &BoundingBox) -> BoundingBox {
    let small = bb1.minimum.0.min(bb2.minimum.0);
    let big = bb1.maximum.0.max(bb2.maximum.0);
    BoundingBox {
      minimum: Point(small),
      maximum: Point(big),
    }
  }

  /*pub fn less_than_by_dim(&self, rhs: &BoundingBox, axis: u32) -> Ordering {
    let sub = self.minimum.0 - rhs.minimum.0;
    let sign = match axis {
        0 => unsafe {
          _mm_cvtss_f32(_mm_shuffle_ps(sub.0, sub.0, _mm_shuffle(0, 0, 0, 3))) },
        1 => unsafe {
          _mm_cvtss_f32(_mm_shuffle_ps(sub.0, sub.0, _mm_shuffle(0, 0, 0, 2))) },
        _ => unsafe {
          _mm_cvtss_f32(_mm_shuffle_ps(sub.0, sub.0, _mm_shuffle(0, 0, 0, 1))) }
    };
    if sign < 0.0 {
      Ordering::Less
    } else if sign == 0.0 {
      Ordering::Equal
    } else {
      Ordering::Greater
    }
  }*/
  pub fn less_than_by_dim(&self, rhs: &BoundingBox, axis: u32) -> Ordering {
    let sub = self.minimum.0 - rhs.minimum.0;
    let sign = match axis {
      0 => { sub.x() },
      1 => { sub.y() },
      _ => { sub.z() }
    };
    if sign < 0.0 {
      Ordering::Less
    } else if sign == 0.0 {
      Ordering::Equal
    } else {
      Ordering::Greater
    }
  }
  
}

const fn _mm_shuffle(z: u32, y: u32, x: u32, w: u32) -> i32 {
    ((z << 6) | (y << 4) | (x << 2) | w) as i32
}
