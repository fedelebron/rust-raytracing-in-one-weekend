mod camera;
mod canvas;
mod material2;
mod object;
mod ray;
mod vec3;
mod aabb;
mod bvh;
mod texture;

use crate::camera::*;
use crate::canvas::*;
use crate::material2::*;
use crate::object::*;
use crate::ray::*;
use crate::vec3::*;
use crate::bvh::*;
use crate::texture::*;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rand::Rng;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;

type T = f32;

fn ray_color(r: &Ray, obj: &dyn Object, depth: i32) -> Color {
  const BLACK: Color = Color::new(0.0, 0.0, 0.0);
  const WHITE: Color = Color::new(1.0, 1.0, 1.0);
  const SKY: Color = Color::new(0.5, 0.7, 1.0);
  if depth <= 0 {
    return BLACK;
  }
  match obj.hit(0.001, T::INFINITY, r) {
    None => {
      let unit = r.direction / r.direction.norm();
      let t = 0.5 * (unit.y + 1.0);
      (1.0 - t) * WHITE + t * SKY
    }
    Some(hr) => match hr.material.scatter(r, &hr) {
      None => BLACK,
      Some(sr) => sr.attenuation * ray_color(&sr.scattered_ray, obj, depth - 1),
    },
  }
}

struct World {
  pub objects: ObjectList,
  pub bvh: BVHNode
}

impl World {
  pub fn new() -> World {
    World {
      objects: ObjectList::new(),
      bvh: BVHNode::new()
    }
  }
  fn create_bvh(&mut self) {
    let n = self.objects.objects.len();
    self.bvh = BVHNode::new_from_objects(
        &mut self.objects.objects[..],
        // &mut (0 .. n - 1).collect::<Vec<usize>>()[..],
        0.0, 1.0)
  }
}

fn make_world() -> World {
  let mut rng = rand::thread_rng();
  let mut world = World::new();
  let checkers = Texture::Checkers(
      Arc::new(Texture::Color(Color::new(0.2, 0.3, 0.1))),
      Arc::new(Texture::Color(Color::new(0.9, 0.9, 0.9))));
  world.objects.add(Box::new(Sphere::new(
    Point::new(0.0, -1000.0, 0.0),
    1000.0,
    Material::new_lambertian(checkers),
  )));

  let image_central_point = Point::new(4.0, 0.2, 0.0);
  for a in -11..11 {
    for b in -11..11 {
      let choose_mat: f32 = rng.gen();
      let sphere_center = Point::new(
        (a as T) + 0.9 * rng.gen::<T>(),
        0.2,
        (b as T) + 0.9 * rng.gen::<T>(),
      );
      if (sphere_center - image_central_point).norm() <= 0.9 {
        continue;
      }
      if choose_mat < 0.5 {
        let center2 = sphere_center; // + Vec3::new(0.0, rng.gen_range(0.0 .. 0.0), 0.0);
        let albedo = Color::random() * Color::random();
        world.objects.add(Box::new(MovingSphere::new(
          sphere_center,
          center2,
          0.0,
          1.0,
          0.2,
          Material::new_lambertian(Texture::Color(albedo)),
        )));
      } else if choose_mat < 0.65 {
        let albedo = Color::random_range(0.5, 1.0);
        let fuzz = rng.gen_range(0.0..0.5);
        world.objects.add(Box::new(Sphere::new(
          sphere_center,
          0.2,
          Material::new_metal(Texture::Color(albedo), fuzz),
        )));
      } else {
        world.objects.add(Box::new(Sphere::new(
          sphere_center,
          0.2,
          Material::new_dielectric(1.5),
        )));
      }
    }
  }

  world.objects.add(Box::new(Sphere::new(
    Point::new(2.0, 1.0, 0.0),
    1.0,
    Material::new_dielectric(1.5),
  )));

  let earth_texture = Texture::from_image_filename("/usr/local/google/home/flebron/rust/sarah.png");
  world.objects.add(Box::new(Sphere::new(
    Point::new(0.0, 1.0, 0.0),
    1.0,
    Material::new_lambertian(earth_texture),
  )));
  world.objects.add(Box::new(Sphere::new(
    Point::new(-2.0, 1.0, 0.0),
    1.0,
    Material::new_metal(Texture::Color(Color::new(0.7, 0.6, 0.5)), 0.0),
  )));

  world.create_bvh();

  return world;
}

fn render_spheres() {
  let image_width = (2 * 400) as u32;
  let image_height = (2 * 225) as u32;

  let samples_per_pixel = 100 as u32;

  let mut canvas = Canvas::new(image_width, image_height);
  //let lookfrom = Point::new(13.0, 2.0, 3.0);
  let lookfrom = Point::new(3.0, 2.0, 13.0);
  let lookat = Point::new(0.0, 0.0, 0.0);
  let vup = Vec3::new(0.0, 1.0, 0.0);
  let aspect_ratio = image_width as T / image_height as T;
  let aperture = 0.05; // 2.0;
  let dist_to_focus = 10.0; // (lookat - lookfrom).norm();
  let camera = Camera::new(
    lookfrom,
    lookat,
    vup,
    20.0,
    aspect_ratio,
    aperture,
    dist_to_focus,
    0.0,
    1.0,
  );

  let world = Arc::new(make_world());

  let n_workers = 8;
  let pool = ThreadPool::new(n_workers);

  let bar = ProgressBar::new((image_height * image_width).into());
  bar.set_style(
    ProgressStyle::default_bar()
      .template("[{percent}%] {wide_bar} {pos:>7}/{len:7} [{elapsed}, ETA: {eta}]"),
  );

  let (tx, rx) = channel();
  for j in 0..image_height {
    let my_world = world.clone();
    let tx = tx.clone();
    pool.execute(move || {
      let mut rng = rand::thread_rng();
      for i in 0..image_width {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _s in 0..samples_per_pixel {
          let ri: f32 = rng.gen();
          let rj: f32 = rng.gen();
          let u = ((i as T) + ri) / (image_width - 1) as T;
          let v = ((j as T) + rj) / (image_height - 1) as T;
          let r = camera.get_ray(u, v);

          pixel_color += ray_color(&r, &my_world.bvh, 50);
        }
        pixel_color /= samples_per_pixel as f32;
        tx.send((i, j, pixel_color)).unwrap();
      }
    })
  }
  drop(tx);

  for (i, j, pixel_color) in rx.iter() {
    bar.inc(1);
    canvas.draw(i, j, &pixel_color);
  }

  pool.join();
  bar.finish();

  canvas.save("scene.png");
}

fn main() {
  render_spheres();
}
