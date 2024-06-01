use super::material_system::Vec2;
use crate::utils::Color;

use dopamine_macros::virtual_method;

use std::mem::MaybeUninit;

#[repr(transparent)]
pub struct RenderView(raw::RenderView);

impl RenderView {
  pub fn set_color(&self, color: &Color) {
    self.as_ref().set_color(color as *const Color as _);
  }

  pub fn color(&self) -> Color {
    let mut out: [MaybeUninit<f32>; 3] = MaybeUninit::uninit_array();
    self.as_ref().color(out.as_mut_ptr().cast());

    unsafe {
      let rgb = MaybeUninit::array_assume_init(out);
      Color::rgb(rgb[0], rgb[1], rgb[2])
    }
  }
}

impl RenderView {
  #[virtual_method(index = 4)]
  fn set_blend(&self, blend: f32);

  #[virtual_method(index = 5)]
  fn blend(&self) -> f32;
}

impl AsRef<raw::RenderView> for RenderView {
  fn as_ref(&self) -> &raw::RenderView {
    &self.0
  }
}

#[repr(C)]
pub struct ViewSetup {
  pad1: [u8; 16],
  width: i32,
  pad2: [u8; 4],
  height: i32,
}

impl ViewSetup {
  pub fn dimensions(&self) -> Vec2<i32> {
    (self.width, self.height)
  }
}

mod raw {
  use dopamine_macros::virtual_method;

  #[repr(C)]
  pub struct RenderView;

  impl RenderView {
    #[virtual_method(index = 6)]
    fn set_color(&self, color: *const f32);

    #[virtual_method(index = 7)]
    fn color(&self, color: *mut f32);
  }
}
