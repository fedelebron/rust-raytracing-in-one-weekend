use crate::aabb::*;
use crate::material2::*;
use crate::ray::*;
use crate::vec3::*;
use std::f32::consts::PI;

type T = f32;

pub struct HitResultPayload<'a> {
  pub p: Point,
  pub normal: Vec3,
  pub front_face: bool,
  pub material: &'a Material,
  // u, v are surface coordinats for this hit.
  pub u: T,
  pub v: T
}

pub struct HitResult<'a> {
  pub t: T,
  pub obj: &'a dyn Object
}

impl HitResult<'_> {
  pub fn new<'a>(
    t: T,
    obj: &'a dyn Object) -> HitResult {
    HitResult { t, obj }
  }
}



impl HitResultPayload<'_> {
  pub fn new<'a>(
    p: Point,
    r: &Ray,
    outward_normal: Vec3,
    material: &'a Material,
    u: T,
    v: T,
  ) -> HitResultPayload<'a> {
    let front_face = r.direction.dot(outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
      } else {
        -outward_normal
      };
    HitResultPayload {
      p,
      normal,
      front_face,
      material,
      u,
      v
    }
  }
}

pub trait Object {
  fn hit(&self, t_min: T, t_max: T, ray: &Ray) -> Option<HitResult>;
  fn hit_payload(&self, t: T, ray: &Ray) -> HitResultPayload;
  fn bounding_box(&self, time0: T, time1: T) -> Option<BoundingBox>;
}

pub struct Sphere {
  center: Point,
  radius: T,
  material: Material,
}

impl Sphere {
  pub fn new(center: Point, radius: T, material: Material) -> Sphere {
    Sphere {
      center,
      radius,
      material,
    }
  }
}

impl Sphere {
}

impl Object for Sphere {
  fn hit(&self, t_min: T, t_max: T, ray: &Ray) -> Option<HitResult> {
    let oc: Vec3 = ray.origin - self.center;
    let a = ray.direction.norm_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.norm_squared() - self.radius * self.radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
      return None;
    }
    let sqrtd = discriminant.sqrt();
    let valid_t = |t: T| -> bool { t >= t_min && t <= t_max };
    let mut root = (-half_b - sqrtd) / a;
    if !valid_t(root) {
      root = (-half_b + sqrtd) / a;
      if !valid_t(root) {
        return None;
      }
    }
    return Some(HitResult { t: root, obj: self });
  }
  fn hit_payload(&self, t: T, ray: &Ray) -> HitResultPayload {
    assert!(t >= 0.0);
    let point = ray.at(t);
    let mut normal = point - self.center;
    normal /= self.radius;

    // If we treat the normal as a point, it becomes the point at which this light ray would've
    // hit the sphere, had the sphere been centered at the origin. We can then use its
    // coordinates to figure out the spherical coordinates for the hit, for such an
    // origin-centered sphere.
    let theta = (-normal.y()).acos();
    let phi = (-normal.z()).atan2(normal.x()) + PI;

    let u = phi / (2.0 * PI);
    let v = theta / PI;
    HitResultPayload::new(point, ray, normal, &self.material, u, v)
  }
  fn bounding_box(&self, _time0: T, _time1: T) -> Option<BoundingBox> {
    let v = Vec3::new(self.radius, self.radius, self.radius);
    Some(BoundingBox::new(
      self.center - v,
      self.center + v
    ))
  }
}

pub struct MovingSphere {
  center0: Point,
  center1: Point,
  time0: T,
  time1: T,
  radius: T,
  material: Material,
}

impl MovingSphere {
  pub fn new(
    center0: Point,
    center1: Point,
    time0: T,
    time1: T,
    radius: T,
    material: Material,
  ) -> MovingSphere {
    MovingSphere {
      center0,
      center1,
      time0,
      time1,
      radius,
      material,
    }
  }
  pub fn center(&self, time: T) -> Point {
    self.center0
      + ((time - self.time0) / (self.time1 - self.time0))
        * (self.center1 - self.center0)
  }
}

impl Object for MovingSphere {
  fn hit(&self, t_min: T, t_max: T, ray: &Ray) -> Option<HitResult> {
    let oc: Vec3 = ray.origin - self.center(ray.time);
    let a = ray.direction.norm_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.norm_squared() - self.radius * self.radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
      return None;
    }
    let sqrtd = discriminant.sqrt();
    let valid_t = |t: T| -> bool { t >= t_min && t <= t_max };
    let mut root = (-half_b - sqrtd) / a;
    if !valid_t(root) {
      root = (-half_b + sqrtd) / a;
      if !valid_t(root) {
        return None;
      }
    }
    Some(HitResult::new(root, self))
  }

  fn hit_payload(&self, t: T, ray: &Ray) -> HitResultPayload {
    assert!(t >= 0.0);
    let point = ray.at(t);
    let normal = (point - self.center(ray.time)) / self.radius;
    // See the Sphere hit subroutine for why we compute u, v this way.
    let theta = (-normal.y()).acos();
    let phi = (-normal.z()).atan2(normal.x()) + PI;

    let u = phi / (2.0 * PI);
    let v = theta / PI;
    HitResultPayload::new(point, ray, normal, &self.material, u, v)
  }
  
  fn bounding_box(&self, time0: T, time1: T) -> Option<BoundingBox> {
    let v = Vec3::new(self.radius, self.radius, self.radius);
    let bb0 = BoundingBox::new(
      self.center(time0) - v,
      self.center(time0) + v
    );
    let bb1 = BoundingBox::new(
      self.center(time1) - v,
      self.center(time1) + v
    );
    Some(BoundingBox::surrounding_box(&bb0, &bb1))
  }
}

pub struct ObjectList {
  pub objects: Vec<Option<Box<dyn Object + Sync + Send>>>,
}

impl ObjectList {
  pub fn new() -> ObjectList {
    ObjectList {
      objects: Vec::new(),
    }
  }
  pub fn add(&mut self, obj: Box<dyn Object + Sync + Send>) {
    self.objects.push(Some(obj));
  }
}
