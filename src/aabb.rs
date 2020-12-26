use crate::ray::*;
use crate::vec3::*;
use std::cmp::Ordering;
use std::mem;

type T = f32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoundingBox {
  minimum: Point,
  maximum: Point,
}

impl BoundingBox {
  pub fn new(minimum: Point, maximum: Point) -> BoundingBox {
    BoundingBox { minimum, maximum }
  }
  pub fn hit(&self, mut t_min: T, mut t_max: T, ray: &Ray) -> bool {
    if false {
        return self.hit_vectorized(t_min, t_max, ray);
    }
    for a in 0..3 {
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
    return true;
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
    let mut small = Point(Vec3::new(0.0, 0.0, 0.0));
    let mut big = Point(Vec3::new(0.0, 0.0, 0.0));
    for a in 0..3 {
      small[a] = bb1.minimum[a].min(bb2.minimum[a]);
      big[a] = bb1.maximum[a].max(bb2.maximum[a]);
    }
    BoundingBox {
      minimum: small,
      maximum: big,
    }
  }

  pub fn less_than_by_dim(&self, rhs: &BoundingBox, axis: usize) -> Ordering {
    self.minimum[axis].partial_cmp(&rhs.minimum[axis]).unwrap()
  }
}
