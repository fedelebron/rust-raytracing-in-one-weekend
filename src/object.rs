use crate::aabb::*;
use crate::material2::*;
use crate::ray::*;
use crate::vec3::*;
use crate::bvh::*;
use std::sync::Arc;
use std::f32::consts::PI;

type T = f32;

pub struct HitResult<'a> {
  pub p: Point,
  pub normal: Vec3,
  pub t: T,
  pub front_face: bool,
  pub material: &'a Material,
  // u, v are surface coordinats for this hit.
  pub u: T,
  pub v: T
}

impl HitResult<'_> {
  pub fn new<'a>(
    p: Point,
    r: &Ray,
    t: T,
    outward_normal: &Vec3,
    material: &'a Material,
    u: T,
    v: T
  ) -> HitResult<'a> {
    let front_face = r.direction.dot(outward_normal) < 0.0;
    let normal = if front_face {
        *outward_normal
      } else {
        -*outward_normal
      };
    HitResult {
      p,
      t,
      normal,
      front_face,
      material,
      u,
      v
    }
  }
}

pub enum Object {
  SphereType(Sphere),
  MovingSphereType(MovingSphere),
  BVHNodeType(BVHNode)
}

impl Object {
  pub fn hit(&self, t_min: T, t_max: T, ray: &Ray) -> Option<HitResult> {
    match self {
      Object::SphereType(ref sphere) => { sphere.hit(t_min, t_max, ray) },
      Object::MovingSphereType(ref moving_sphere) => { moving_sphere.hit(t_min, t_max, ray) },
      Object::BVHNodeType(ref bvhnode) => { bvhnode.hit(t_min, t_max, ray) }
    }
  }
  pub fn bounding_box(&self, time0: T, time1: T) -> Option<BoundingBox> {
    match self {
      Object::SphereType(ref sphere) => { sphere.bounding_box(time0, time1) },
      Object::MovingSphereType(ref moving_sphere) => { moving_sphere.bounding_box(time0, time1) },
      Object::BVHNodeType(ref bvhnode) => { bvhnode.bounding_box(time0, time1) }
    }
  }
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
  fn hits_sphere(&self, t_min: T, t_max: T, ray: &Ray) -> T {
    let oc: Vec3 = ray.origin - self.center;
    let a = ray.direction.norm_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.norm_squared() - self.radius * self.radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
      return -1.0;
    }
    let sqrtd = discriminant.sqrt();
    let valid_t = |t: T| -> bool { t >= t_min && t <= t_max };
    let mut root = (-half_b - sqrtd) / a;
    if !valid_t(root) {
      root = (-half_b + sqrtd) / a;
      if !valid_t(root) {
        return -1.0;
      }
    }
    return root;
  }
  
  pub fn hit(&self, t_min: T, t_max: T, ray: &Ray) -> Option<HitResult> {
    let t = self.hits_sphere(t_min, t_max, ray);
    if t < 0.0 {
      None
    } else {
      let point = ray.at(t);
      let mut normal = point - self.center;
      normal /= self.radius;

      // If we treat the normal as a point, it becomes the point at which this light ray would've
      // hit the sphere, had the sphere been centered at the origin. We can then use its
      // coordinates to figure out the spherical coordinates for the hit, for such an
      // origin-centered sphere.
      let theta = (-normal.y).acos();
      let phi = (-normal.z).atan2(normal.x) + PI;

      let u = phi / (2.0 * PI);
      let v = theta / PI;
      Some(HitResult::new(point, ray, t, &normal, &self.material, u, v))
    }
  }
  pub fn bounding_box(&self, time0: T, time1: T) -> Option<BoundingBox> {
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

impl MovingSphere {
  pub fn hit(&self, t_min: T, t_max: T, ray: &Ray) -> Option<HitResult> {
    let oc: Vec3 = ray.origin - self.center(ray.time);
    let a = ray.direction.norm_squared();
    let half_b = oc.dot(&ray.direction);
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
    let t = root;
    if t < 0.0 {
      None
    } else {
      let point = ray.at(t);
      let normal = (point - self.center(ray.time)) / self.radius;
      // See the Sphere hit subroutine for why we compute u, v this way.
      let theta = (-normal.y).acos();
      let phi = (-normal.z).atan2(normal.x) + PI;

      let u = phi / (2.0 * PI);
      let v = theta / PI;
      Some(HitResult::new(point, ray, t, &normal, &self.material, u, v))
    }
  }
  
  pub fn bounding_box(&self, time0: T, time1: T) -> Option<BoundingBox> {
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
  pub objects: Vec<Option<Box<Object>>>,
}

impl ObjectList {
  pub fn new() -> ObjectList {
    ObjectList {
      objects: Vec::new(),
    }
  }
  pub fn add(&mut self, obj: Box<Object>) {
    self.objects.push(Some(obj));
  }
}
/*
impl Object for ObjectList {
  fn hit(&self, t_min: T, t_max: T, ray: &Ray) -> Option<HitResult> {
    let mut result: Option<HitResult> = None;
    let mut current_max = t_max;
    for obj in self.objects.iter() {
      match obj.as_ref().unwrap().hit(t_min, current_max, ray) {
        Some(hr) => {
          current_max = hr.t;
          result = Some(hr);
        }
        None => {}
      }
    }
    return result;
  }
  fn bounding_box(&self, time0: T, time1: T) -> Option<BoundingBox> {
    let mut bbox = None;
    for obj in self.objects.iter() {
      match obj.as_ref().unwrap().bounding_box(time0, time1) {
        None => {}
        Some(obj_box) => { 
          match bbox {
            None => { bbox = Some(obj_box); }
            Some(ref mut x) => { *x = BoundingBox::surrounding_box(&x, &obj_box); }
          }
        }
      }
    }
    bbox
  }
}*/
