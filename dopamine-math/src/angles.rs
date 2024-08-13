use crate::Vector;

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
