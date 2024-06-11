use num_traits::{AsPrimitive, NumCast};
use serde::{Deserialize, Serialize};

use std::mem;
use std::ops::Mul;

#[derive(Clone, Copy, Default)]
enum ColorMode {
  #[default]
  OneBased,
  FullByte,
  Undefined,
}

/// Holds 4 components with type `T`. Each component represents one color attribute.
///
/// Supports serializing and deserializing using crate-`serde`.
/// Has C-compatible memory layout.
#[derive(Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Color<T = f32>
where
  T: NumCast + Clone + Copy,
{
  pub r: T,
  pub g: T,
  pub b: T,
  pub a: T,
  #[serde(skip)]
  mode: ColorMode,
}

impl<T: NumCast + Clone + Copy> Color<T> {
  fn rgbam(r: T, g: T, b: T, a: T, mode: ColorMode) -> Self {
    Self { r, g, b, a, mode }
  }

  pub fn rgba(r: T, g: T, b: T, a: T) -> Self {
    Self::rgbam(r, g, b, a, ColorMode::default())
  }

  pub fn rgb(r: T, g: T, b: T) -> Self {
    Self::rgba(r, g, b, unsafe { T::from(1.0).unwrap_unchecked() })
  }

  pub fn white() -> Self {
    unsafe {
      Self::rgba(
        T::from(1.0).unwrap_unchecked(),
        T::from(1.0).unwrap_unchecked(),
        T::from(1.0).unwrap_unchecked(),
        T::from(1.0).unwrap_unchecked(),
      )
    }
  }

  pub fn black() -> Self {
    unsafe {
      Self::rgba(
        T::from(0.0).unwrap_unchecked(),
        T::from(0.0).unwrap_unchecked(),
        T::from(0.0).unwrap_unchecked(),
        T::from(1.0).unwrap_unchecked(),
      )
    }
  }

  pub fn as_mut_array(&mut self) -> &mut [T; 4] {
    unsafe { mem::transmute(self) }
  }
}

impl<T> Color<T>
where
  T: NumCast + Clone + Copy + AsPrimitive<u8> + Mul + 'static,
  <T as Mul>::Output: AsPrimitive<T>,
{
  pub fn mul_255_if_supports(self) -> Self {
    match self.mode {
      ColorMode::OneBased => self * unsafe { T::from(255).unwrap_unchecked() },
      ColorMode::FullByte | ColorMode::Undefined => self,
    }
  }
}

impl<T> Mul<T> for Color<T>
where
  T: NumCast + Clone + Copy + AsPrimitive<u8> + Mul + 'static,
  <T as Mul>::Output: AsPrimitive<T>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    let mode = if rhs.as_() == 255 {
      ColorMode::FullByte
    } else {
      ColorMode::Undefined
    };

    Self::rgbam(
      (self.r * rhs).as_(),
      (self.g * rhs).as_(),
      (self.b * rhs).as_(),
      (self.a * rhs).as_(),
      mode,
    )
  }
}
