use crate::object::*;
use crate::ray::*;
use crate::texture::*;
use crate::vec3::*;
use rand::Rng;
type T = f32;

pub struct ScatterResult {
  pub attenuation: Color,
  pub scattered_ray: Ray,
}

#[derive(Clone)]
pub enum Material {
  Lambertian { albedo: Texture },
  Metal { albedo: Texture, fuzz: T },
  Dielectric { refraction_index: T },
}

impl Material {
  pub fn scatter(
    &self,
    incident_ray: &Ray,
    hit: &HitResultPayload,
  ) -> Option<ScatterResult> {
    match self {
      Material::Lambertian { albedo: a } => {
        scatter_lambertian(a, incident_ray, hit)
      }
      Material::Metal { albedo: a, fuzz: f } => {
        scatter_metal(a, *f, incident_ray, hit)
      }
      Material::Dielectric {
        refraction_index: ir,
      } => scatter_dielectric(*ir, incident_ray, hit),
    }
  }
  pub fn new_lambertian(albedo: Texture) -> Material {
    Material::Lambertian { albedo }
  }
  pub fn new_metal(albedo: Texture, fuzz: T) -> Material {
    Material::Metal {
      albedo,
      fuzz: fuzz.min(1.0),
    }
  }
  pub fn new_dielectric(refraction_index: T) -> Material {
    Material::Dielectric { refraction_index }
  }
}

fn scatter_lambertian(
  albedo: &Texture,
  incident_ray: &Ray,
  hit: &HitResultPayload,
) -> Option<ScatterResult> {
  let mut scatter_direction = hit.normal + Vec3::random_unit();
  if scatter_direction.near_zero() {
    scatter_direction = hit.normal;
  }
  Some(ScatterResult {
    attenuation: albedo.value(hit.u, hit.v, &hit.p),
    scattered_ray: Ray {
      origin: hit.p,
      direction: scatter_direction,
      time: incident_ray.time,
    },
  })
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
  *v - 2.0 * (*v).dot(n) * (*n)
}

fn refract(v: &Vec3, n: &Vec3, refraction_ratio: T) -> Vec3 {
  let cos_theta = (-(*v).dot(n)).min(1.0);
  let r_out_perp = refraction_ratio * ((*v) + cos_theta * (*n));
  let r_out_parallel =
    (1.0 - r_out_perp.norm_squared()).abs().sqrt() * (*n) * (-1.0);
  r_out_perp + r_out_parallel
}

fn scatter_metal(
  albedo: &Texture,
  fuzz: T,
  incident_ray: &Ray,
  hit: &HitResultPayload,
) -> Option<ScatterResult> {
  let r = incident_ray.direction;
  let reflected = reflect(&(r / r.norm()), &hit.normal);
  let scattered = Ray {
    origin: hit.p,
    direction: reflected + fuzz * Vec3::random_unit(),
    time: incident_ray.time,
  };
  if reflected.dot(&hit.normal) > 0.0 {
    return Some(ScatterResult {
      attenuation: albedo.value(hit.u, hit.v, &hit.p),
      scattered_ray: scattered,
    });
  }
  return None;
}

fn scatter_dielectric(
  refraction_index: T,
  incident_ray: &Ray,
  hit: &HitResultPayload,
) -> Option<ScatterResult> {
  let attenuation = Color::new(1.0, 1.0, 1.0);
  let refraction_ratio = if hit.front_face {
    1.0 / refraction_index
  } else {
    refraction_index
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
  let direction =
    if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen() {
      reflect(&unit_direction, &hit.normal)
    } else {
      refract(&unit_direction, &hit.normal, refraction_ratio)
    };

  let scattered = Ray {
    origin: hit.p,
    direction: direction,
    time: incident_ray.time,
  };
  Some(ScatterResult {
    attenuation: attenuation,
    scattered_ray: scattered,
  })
}
