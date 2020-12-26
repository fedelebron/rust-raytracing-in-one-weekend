use rand::Rng;
use std::f32::consts::PI;
use std::ops::{
  Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

type T = f32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl Vec3 {
  pub fn new(x: T, y: T, z: T) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn random_unit() -> Vec3 {
    let mut rng = rand::thread_rng();
    let theta = (2.0 * PI * rng.gen::<T>()) as T;
    let phi = ((1.0 - 2.0 * rng.gen::<T>()) as T).acos();
    Vec3 {
      x: phi.sin() * theta.cos(),
      y: phi.sin() * theta.sin(),
      z: phi.cos(),
    }
  }

  pub fn near_zero(&self) -> bool {
    for i in 0..3 {
      if self[i].abs() > 1e-8 {
        return false;
      }
    }
    return true;
  }

  pub fn norm_squared(&self) -> T {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn norm(&self) -> T {
    self.norm_squared().sqrt()
  }

  pub fn normalize(&self) -> Vec3 {
    *self / self.norm()
  }

  pub fn cross(&self, v: &Vec3) -> Vec3 {
    Vec3::new(
      self.y * v.z - self.z * v.y,
      self.z * v.x - self.x * v.z,
      self.x * v.y - self.y * v.x,
    )
  }

  pub fn dot(&self, rhs: &Vec3) -> T {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
  }

  pub fn min(&self, rhs: Vec3) -> Vec3 {
    Vec3 {
        x: self.x.min(rhs.x),
        y: self.y.min(rhs.y),
        z: self.z.min(rhs.z)
    }
  }
  
  pub fn max(&self, rhs: Vec3) -> Vec3 {
    Vec3 {
        x: self.x.max(rhs.x),
        y: self.y.max(rhs.y),
        z: self.z.max(rhs.z)
    }
  }

  pub fn min_element(&self) -> T {
    self.x.min(self.y.min(self.z))
  }
  
  pub fn max_element(&self) -> T {
    self.x.max(self.y.max(self.z))
  }
}

impl Index<usize> for Vec3 {
  type Output = T;

  fn index(&self, index: usize) -> &T {
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Invalid vec3 index: {:?}", index),
    }
  }
}

impl IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      _ => panic!("Invalid vec3 index: {:?}", index),
    }
  }
}

impl Neg for Vec3 {
  type Output = Self;
  fn neg(self) -> Self {
    Vec3 {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl Add<Vec3> for Vec3 {
  type Output = Self;
  fn add(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl AddAssign<Vec3> for Vec3 {
  fn add_assign(&mut self, rhs: Vec3) {
    self.x += rhs.x;
    self.y += rhs.y;
    self.z += rhs.z;
  }
}

impl Sub<Vec3> for Vec3 {
  type Output = Self;
  fn sub(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl SubAssign<Vec3> for Vec3 {
  fn sub_assign(&mut self, rhs: Vec3) {
    self.x -= rhs.x;
    self.y -= rhs.y;
    self.z -= rhs.z;
  }
}

impl Mul<T> for Vec3 {
  type Output = Self;
  fn mul(self, rhs: T) -> Vec3 {
    Vec3 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl Mul<Vec3> for Vec3 {
  type Output = Self;
  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z,
    }
  }
}

impl Mul<Vec3> for T {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Vec3 {
    rhs * self
  }
}

impl MulAssign<T> for Vec3 {
  fn mul_assign(&mut self, rhs: T) {
    self.x *= rhs;
    self.y *= rhs;
    self.z *= rhs;
  }
}

impl Div<T> for Vec3 {
  type Output = Self;
  fn div(self, rhs: T) -> Vec3 {
    Vec3 {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    }
  }
}

impl Div<Vec3> for T {
  type Output = Vec3;
  fn div(self, rhs: Vec3) -> Vec3 {
    Vec3 {
      x: self / rhs.x,
      y: self / rhs.y,
      z: self / rhs.z
    }
  }
}

impl DivAssign<T> for Vec3 {
  fn div_assign(&mut self, rhs: T) {
    self.x /= rhs;
    self.y /= rhs;
    self.z /= rhs;
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(pub Vec3);
impl Point {
  pub fn new(x: T, y: T, z: T) -> Point {
    Point(Vec3 { x: x, y: y, z: z })
  }
}

impl Sub<Vec3> for Point {
  type Output = Self;
  fn sub(self, rhs: Vec3) -> Point {
    Point(Vec3 {
      x: self.0.x - rhs.x,
      y: self.0.y - rhs.y,
      z: self.0.z - rhs.z,
    })
  }
}

impl Sub<Point> for Point {
  type Output = Vec3;
  fn sub(self, rhs: Point) -> Vec3 {
    Vec3 {
      x: self.0.x - rhs.0.x,
      y: self.0.y - rhs.0.y,
      z: self.0.z - rhs.0.z,
    }
  }
}

impl Add<Vec3> for Point {
  type Output = Self;
  fn add(self, rhs: Vec3) -> Point {
    Point(Vec3 {
      x: self.0.x + rhs.x,
      y: self.0.y + rhs.y,
      z: self.0.z + rhs.z,
    })
  }
}

impl Add<Point> for Point {
  type Output = Self;
  fn add(self, rhs: Point) -> Point {
    Point(self.0 + rhs.0)
  }
}

impl Mul<T> for Point {
  type Output = Self;
  fn mul(self, rhs: T) -> Self {
    Point(Vec3 {
        x: self.0.x * rhs,
        y: self.0.y * rhs,
        z: self.0.z * rhs
    })
  }
}

impl Mul<Point> for T {
  type Output = Point;
  fn mul(self, rhs: Point) -> Point {
    rhs * self
  }
}

impl Index<usize> for Point {
  type Output = T;
  fn index(&self, index: usize) -> &T {
    &self.0[index]
  }
}

impl IndexMut<usize> for Point {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.0[index]
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(pub Vec3);
impl Color {
  pub const fn new(x: T, y: T, z: T) -> Color {
    Color(Vec3 { x: x, y: y, z: z })
  }

  pub fn random() -> Color {
    let mut rng = rand::thread_rng();
    Color::new(rng.gen(), rng.gen(), rng.gen())
  }

  pub fn random_range(lo: T, hi: T) -> Color {
    let mut rng = rand::thread_rng();
    Color::new(
      rng.gen_range(lo..hi),
      rng.gen_range(lo..hi),
      rng.gen_range(lo..hi),
    )
  }

  pub fn r(&self) -> T {
    self.0.x
  }
  pub fn g(&self) -> T {
    self.0.y
  }
  pub fn b(&self) -> T {
    self.0.z
  }
}
unsafe impl Sync for Color {}
unsafe impl Send for Color {}
impl Mul<T> for Color {
  type Output = Self;
  fn mul(self, rhs: T) -> Color {
    Color(self.0 * rhs)
  }
}

impl Mul<Color> for T {
  type Output = Color;
  fn mul(self, rhs: Color) -> Color {
    Color(rhs.0 * self)
  }
}

impl Mul<Color> for Color {
  type Output = Color;
  fn mul(self, rhs: Color) -> Color {
    Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
  }
}

impl Add<Color> for Color {
  type Output = Color;
  fn add(self, rhs: Color) -> Color {
    Color(self.0 + rhs.0)
  }
}

impl AddAssign<Color> for Color {
  fn add_assign(&mut self, rhs: Color) {
    self.0 += rhs.0;
  }
}

impl DivAssign<T> for Color {
  fn div_assign(&mut self, rhs: T) {
    self.0 /= rhs;
  }
}
