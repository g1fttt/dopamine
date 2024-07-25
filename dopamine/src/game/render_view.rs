use crate::utils::Color;

use dopamine_macros::virtual_method;

use std::mem::MaybeUninit;

#[repr(C)]
pub struct RenderView;

impl RenderView {
  #[inline]
  pub fn set_color(&self, color: &Color) {
    self.set_color_raw(color as *const Color as _);
  }

  pub fn color(&self) -> Color {
    let mut out: [MaybeUninit<f32>; 3] = MaybeUninit::uninit_array();
    self.color_raw(out.as_mut_ptr().cast());

    unsafe {
      let rgb = MaybeUninit::array_assume_init(out);
      Color::rgb(rgb[0], rgb[1], rgb[2])
    }
  }
}

impl RenderView {
  virtual_method!(pub fn set_blend[4](&self, blend: f32));
  virtual_method!(pub fn blend[5](&self) -> f32);
}

impl RenderView {
  virtual_method!(fn set_color_raw[6](&self, color: *const f32));
  virtual_method!(fn color_raw[7](&self, color: *mut f32));
}

#[repr(C)]
pub struct ViewSetup {
  pad1: [u8; 16],
  width: i32,
  pad2: [u8; 4],
  height: i32,
  pad3: [u8; 25],
  pub fov: f32,
}

impl ViewSetup {
  #[inline]
  pub fn dimensions(&self) -> (i32, i32) {
    (self.width, self.height)
  }
}
