use crate::ray::*;
use crate::vec3::*;
use rand::Rng;
use std::f32::consts::PI;
type T = f32;

#[derive(Copy, Clone)]
pub struct Camera {
  origin: Point,
  lower_left_corner: Point,
  horizontal: Vec3,
  vertical: Vec3,
  // u, v, w are an orthonormal basis for R^3, and define the
  // camera's orientation.
  //   * w points towards the object
  //   * u, v form the basis of the plane tangential to the lens.
  u: Vec3,
  v: Vec3,
  _w: Vec3,
  lens_radius: T,
  time0: T,
  time1: T
}

// S^1 here is embedded into R^3 by the mapping (x, y) -> (x, y, 0).
fn random_vector_in_s1() -> (T, T) {
  let mut rng = rand::thread_rng();
  let theta = rng.gen::<T>() * 2.0 * PI;
  (theta.cos(), theta.sin())
}

impl Camera {
  pub fn new(
    lookfrom: Point,
    lookat: Point,
    vup: Vec3,
    vfov: T,
    aspect_ratio: T,
    aperture: T,
    focus_distance: T,
    time0: T,
    time1: T
  ) -> Camera {
    let theta = vfov.to_radians();
    let h = (theta / 2.0).tan();

    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = (lookfrom - lookat).normalize();
    let u = vup.cross(w);
    let v = w.cross(u);

    let origin = lookfrom;
    let horizontal = focus_distance * viewport_width * u;
    let vertical = focus_distance * viewport_height * v;
    let towards_camera = focus_distance * w;

    Camera {
      origin: origin,
      lower_left_corner: origin - towards_camera - horizontal / 2.0 - vertical / 2.0,
      horizontal: horizontal,
      vertical: vertical,
      u: u,
      v: v,
      _w: w,
      lens_radius: aperture / 2.0,
      time0: time0,
      time1: time1
    }
  }

  pub fn get_ray(&self, s: T, t: T) -> Ray {
    let mut rng = rand::thread_rng();
    let (vx, vy) = random_vector_in_s1();
    let offset = self.lens_radius * (self.u * vx + self.v * vy);
    Ray {
      origin: self.origin + offset,
      direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
        - self.origin
        - offset,
      time: rng.gen_range(self.time0 .. self.time1)
    }
  }
}
