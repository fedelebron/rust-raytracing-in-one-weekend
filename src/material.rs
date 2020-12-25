use crate::object::*;
use crate::ray::*;
use crate::vec3::*;
use rand::Rng;
type T = f32;

pub struct ScatterResult {
  pub attenuation: Color,
  pub scattered_ray: Ray,
}

pub trait Material {
  fn scatter(&self, incident_ray: &Ray, hit: &HitResult) -> Option<ScatterResult>;
}

pub struct Lambertian {
  pub albedo: Color,
}

impl Lambertian {
  pub fn new(albedo: Color) -> Lambertian {
    Lambertian { albedo: albedo }
  }
}

impl Material for Lambertian {
  fn scatter(&self, _incident_ray: &Ray, hit: &HitResult) -> Option<ScatterResult> {
    let mut scatter_direction = hit.normal + Vec3::random_unit();
    if scatter_direction.near_zero() {
      scatter_direction = hit.normal;
    }
    Some(ScatterResult {
      attenuation: self.albedo,
      scattered_ray: Ray {
        origin: hit.p,
        direction: scatter_direction,
      },
    })
  }
}

pub struct Metal {
  albedo: Color,
  fuzz: T,
}

impl Metal {
  pub fn new(albedo: Color, fuzz: T) -> Metal {
    Metal {
      albedo: albedo,
      fuzz: fuzz.min(1.0),
    }
  }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
  *v - 2.0 * (*v).dot(n) * (*n)
}

fn refract(v: &Vec3, n: &Vec3, refraction_ratio: T) -> Vec3 {
  let cos_theta = (-(*v).dot(n)).min(1.0);
  let r_out_perp = refraction_ratio * ((*v) + cos_theta * (*n));
  let r_out_parallel = (1.0 - r_out_perp.norm_squared()).abs().sqrt() * (*n) * (-1.0);
  r_out_perp + r_out_parallel
}

impl Material for Metal {
  fn scatter(&self, incident_ray: &Ray, hit: &HitResult) -> Option<ScatterResult> {
    let r = incident_ray.direction;
    let reflected = reflect(&(r / r.norm()), &hit.normal);
    let scattered = Ray {
      origin: hit.p,
      direction: reflected + self.fuzz * Vec3::random_unit(),
    };
    if reflected.dot(&hit.normal) > 0.0 {
      return Some(ScatterResult {
        attenuation: self.albedo,
        scattered_ray: scattered,
      });
    }
    return None;
  }
}

pub struct Dielectric {
  refraction_index: T,
}

impl Dielectric {
  pub fn new(ir: T) -> Dielectric {
    Dielectric {
      refraction_index: ir,
    }
  }
}

impl Material for Dielectric {
  fn scatter(&self, incident_ray: &Ray, hit: &HitResult) -> Option<ScatterResult> {
    let attenuation = Color::new(1.0, 1.0, 1.0);
    let refraction_ratio = if hit.front_face {
      1.0 / self.refraction_index
    } else {
      self.refraction_index
    };

    let r = incident_ray.direction;
    let unit_direction = r / r.norm();

    let cos_theta = (-unit_direction.dot(&hit.normal)).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    let cannot_refract = refraction_ratio * sin_theta > 1.0;

    let mut rng = rand::thread_rng();
    let reflectance = |cosine: T, refraction_ratio: T| -> T {
      let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio).powf(2.0);
      r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    };
    let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen() {
      reflect(&unit_direction, &hit.normal)
    } else {
      refract(&unit_direction, &hit.normal, refraction_ratio)
    };

    let scattered = Ray {
      origin: hit.p,
      direction: direction,
    };
    Some(ScatterResult {
      attenuation: attenuation,
      scattered_ray: scattered,
    })
  }
}
