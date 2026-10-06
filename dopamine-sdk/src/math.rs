use serde::{Deserialize, Serialize};

use std::ops::{Add, Index, Mul, Sub};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec3 {
  pub const fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }

  pub fn cross_product(&self, other: &Vec3) -> Self {
    Self {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }

  pub fn dot_product(&self, other: &Vec3) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  pub fn transform(&self, mat: &Mat4x4) -> Self {
    Self {
      x: self.dot_product(&Self::new(mat[0][0], mat[0][1], mat[0][2])) + mat[0][3],
      y: self.dot_product(&Self::new(mat[1][0], mat[1][1], mat[1][2])) + mat[1][3],
      z: self.dot_product(&Self::new(mat[2][0], mat[2][1], mat[2][2])) + mat[2][3],
    }
  }
}

impl Mul<f32> for Vec3 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Self::Output { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
  }
}

fn add_two_vectors(a: &Vec3, b: &Vec3) -> Vec3 {
  Vec3 { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z }
}

impl Add<Vec3> for Vec3 {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: Vec3) -> Self::Output {
    add_two_vectors(&self, &rhs)
  }
}

impl Add<Vec3> for &Vec3 {
  type Output = Vec3;

  #[inline(always)]
  fn add(self, rhs: Vec3) -> Self::Output {
    add_two_vectors(self, &rhs)
  }
}

fn sub_two_vectors(a: &Vec3, b: &Vec3) -> Vec3 {
  Vec3 { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z }
}

impl Sub<Vec3> for Vec3 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, rhs: Vec3) -> Self::Output {
    sub_two_vectors(&self, &rhs)
  }
}

impl Sub<Vec3> for &Vec3 {
  type Output = Vec3;

  #[inline(always)]
  fn sub(self, rhs: Vec3) -> Self::Output {
    sub_two_vectors(self, &rhs)
  }
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct Angles {
  pub yaw: f32,
  pub pitch: f32,
  pub roll: f32,
}

impl Angles {
  pub const fn new(yaw: f32, pitch: f32, roll: f32) -> Self {
    Self { yaw, pitch, roll }
  }

  pub fn to_vector(&self) -> Vec3 {
    let yaw = self.yaw.to_radians();
    let pitch = self.pitch.to_radians();

    Vec3::new(yaw.cos() * pitch.cos(), yaw.cos() * pitch.sin(), -yaw.sin())
  }

  #[inline(always)]
  pub fn forward_vector(&self) -> Vec3 {
    self.to_vector()
  }

  pub fn up_vector(&self) -> Vec3 {
    Self::new(self.yaw - 90.0, self.pitch, self.roll).to_vector()
  }
}

#[derive(Debug)]
#[repr(C)]
pub struct Mat4x4 {
  inner: MatN<4, 4>,
}

impl Index<usize> for Mat4x4 {
  type Output = Row<4>;

  fn index(&self, index: usize) -> &Self::Output {
    &self.inner[index]
  }
}

pub type Row<const N: usize> = [f32; N];

#[derive(Debug)]
#[repr(C)]
struct MatN<const R: usize, const C: usize> {
  data: [Row<C>; R],
}

impl<const R: usize, const C: usize> Index<usize> for MatN<R, C> {
  type Output = Row<C>;

  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}
