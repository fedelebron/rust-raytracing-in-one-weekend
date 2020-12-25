use crate::vec3::*;

type T = f32;

pub struct Ray {
  pub origin: Point,
  pub direction: Vec3,
  pub time: T
}

impl Ray {
  pub fn new(origin: Point, direction: Vec3, time: T) -> Ray {
    Ray { origin, direction, time }
  }
  pub fn at(&self, t: T) -> Point {
    Point(self.origin.0 + t * self.direction)
  }
}
