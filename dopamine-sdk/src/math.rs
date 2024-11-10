use serde::{Deserialize, Serialize};

use std::ops::{Add, Mul};

#[derive(Default, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Vector {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }

  pub fn cross_product(&self, other: &Vector) -> Self {
    Self {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }
}

impl Mul<f32> for Vector {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Self::Output { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
  }
}

fn add_two_vectors(a: &Vector, b: &Vector) -> Vector {
  Vector { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z }
}

impl Add<Vector> for Vector {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: Vector) -> Self::Output {
    add_two_vectors(&self, &rhs)
  }
}

impl Add<Vector> for &Vector {
  type Output = Vector;

  #[inline(always)]
  fn add(self, rhs: Vector) -> Self::Output {
    add_two_vectors(self, &rhs)
  }
}

#[repr(C)]
pub struct Angles {
  pub yaw: f32,
  pub pitch: f32,
  pub roll: f32,
}

impl Angles {
  pub fn new(yaw: f32, pitch: f32, roll: f32) -> Self {
    Self { yaw, pitch, roll }
  }

  pub fn to_vector(&self) -> Vector {
    let yaw = self.yaw.to_radians();
    let pitch = self.pitch.to_radians();

    Vector::new(yaw.cos() * pitch.cos(), yaw.cos() * pitch.sin(), -yaw.sin())
  }

  #[inline(always)]
  pub fn forward_vector(&self) -> Vector {
    self.to_vector()
  }

  pub fn up_vector(&self) -> Vector {
    Self::new(self.yaw - 90.0, self.pitch, self.roll).to_vector()
  }
}
