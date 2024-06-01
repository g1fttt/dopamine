use super::KeyValues;
use crate::cstr;
use crate::utils::Color;

use derive_builder::Builder;
use dopamine_macros::virtual_method;

use std::ptr;

pub type Vec2<T> = (T, T);

#[repr(C)]
pub enum MaterialFlag {
  IgnoreZ = 1 << 15,
}

#[repr(C)]
pub struct Material;

impl Material {
  #[virtual_method(index = 29)]
  fn set_flag(&self, flag: MaterialFlag, state: bool);
}

#[repr(transparent)]
pub struct Texture(private::Texture);

impl Texture {
  pub fn dimensions(&self) -> (i32, i32) {
    let raw_texture = self.as_ref();
    (raw_texture.actual_width(), raw_texture.actual_height())
  }
}

impl Texture {
  #[virtual_method(index = 10)]
  fn inc_ref_counter(&self);
}

impl AsRef<private::Texture> for Texture {
  fn as_ref(&self) -> &private::Texture {
    &self.0
  }
}

#[repr(transparent)]
pub struct MaterialSystem(private::MaterialSystem);

impl MaterialSystem {
  pub fn create_material(&self, name: &str, kv: &KeyValues) -> Option<&Material> {
    self.as_ref().create_material(cstr!(name), kv)
  }

  pub fn find_texture(&self, name: &str, group: &str) -> Option<&Texture> {
    self
      .as_ref()
      .find_texture(cstr!(name), cstr!(group), true, 0)
  }

  pub fn create_named_rt(&self, name: &str, dimensions: Vec2<i32>) -> Option<&Texture> {
    self
      .as_ref()
      .create_named_rt_ex(cstr!(name), dimensions.0, dimensions.1, 1, 0, 1, 0x200C, 1)
  }
}

impl MaterialSystem {
  #[virtual_method(index = 98)]
  fn render_ctx(&self) -> &RenderContext;
}

impl AsRef<private::MaterialSystem> for MaterialSystem {
  fn as_ref(&self) -> &private::MaterialSystem {
    &self.0
  }
}

#[derive(Default, Clone, Copy)]
#[repr(C)]
pub enum StencilOp {
  #[default]
  Keep = 1,
  Replace = 3,
}

#[derive(Default, Clone, Copy)]
#[repr(C)]
pub enum StencilCmpFn {
  Equal = 3,
  #[default]
  Always = 8,
}

#[repr(transparent)]
pub struct RenderContext(private::RenderContext);

impl RenderContext {
  pub fn push_rt_and_set_viewport(&self, rt: &Texture, dimensions: Vec2<i32>) {
    self.push_rt_and_viewport(rt);
    self.set_viewport((0, 0), dimensions);
  }

  pub fn clear_color_3u8(&self, color: Color<u8>) {
    let color = color.mul_255_if_supports();
    self.as_ref().clear_color_3u8(color.r, color.g, color.b);
  }

  fn set_viewport(&self, pos: Vec2<i32>, dimensions: Vec2<i32>) {
    self
      .as_ref()
      .set_viewport(pos.0, pos.1, dimensions.0, dimensions.1);
  }
}

impl RenderContext {
  #[virtual_method(index = 6)]
  fn set_render_target(&self, texture: &Texture);

  #[virtual_method(index = 107)]
  fn push_rt_and_viewport(&self, rt: &Texture);

  #[virtual_method(index = 109)]
  fn pop_rt_and_viewport(&self);

  #[virtual_method(index = 117)]
  fn set_stencil_enable(&self, enable: bool);

  #[virtual_method(index = 118)]
  fn set_stencil_fail_op(&self, op: StencilOp);

  #[virtual_method(index = 119)]
  fn set_stencil_z_fail_op(&self, op: StencilOp);

  #[virtual_method(index = 120)]
  fn set_stencil_pass_op(&self, op: StencilOp);

  #[virtual_method(index = 121)]
  fn set_stencil_cmp_fn(&self, cmp_fn: StencilCmpFn);

  #[virtual_method(index = 122)]
  fn set_stencil_ref_value(&self, ref_value: i32);

  #[virtual_method(index = 123)]
  fn set_stencil_test_mask(&self, mask: u32);

  #[virtual_method(index = 124)]
  fn set_stencil_write_mask(&self, mask: u32);
}

impl AsRef<private::RenderContext> for RenderContext {
  fn as_ref(&self) -> &private::RenderContext {
    &self.0
  }
}

#[derive(Builder)]
#[builder(pattern = "owned", derive(Clone))]
pub struct ScreenSpaceRect<'a> {
  material: &'a Material,
  pos: Vec2<i32>,
  dimensions: Vec2<i32>,
  texture_x0_y0: Vec2<f32>,
  texture_x1_y1: Vec2<f32>,
  texture_dimensions: Vec2<i32>,
}

impl ScreenSpaceRectBuilder<'_> {
  pub fn build_and_draw(self, render_ctx: &RenderContext) {
    let rect = self.build().expect("Failed to build ScreenSpaceRect");
    render_ctx.as_ref().draw_screen_space_rect(
      rect.material,
      rect.pos.0,
      rect.pos.1,
      rect.dimensions.0,
      rect.dimensions.1,
      rect.texture_x0_y0.0,
      rect.texture_x0_y0.1,
      rect.texture_x1_y1.0,
      rect.texture_x1_y1.1,
      rect.texture_dimensions.0,
      rect.texture_dimensions.1,
      ptr::null_mut(),
      1,
      1,
    );
  }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ClearBuffers {
  clear_color: bool,
  clear_depth: bool,
  #[builder(default)]
  clear_stencil: bool,
}

impl ClearBuffersBuilder {
  pub fn build_and_clear(self, render_ctx: &RenderContext) {
    let buffers = self.build().expect("Failed to build ClearBuffers");
    render_ctx.as_ref().clear_buffers(
      buffers.clear_color,
      buffers.clear_depth,
      buffers.clear_stencil,
    );
  }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct OverrideDepth {
  enable: bool,
  #[builder(default)]
  depth_enable: bool,
}

impl OverrideDepthBuilder {
  pub fn build_and_override(self, render_ctx: &RenderContext) {
    let overrides = self.build().expect("Failed to build OverrideDepth");
    render_ctx
      .as_ref()
      .override_depth_enable(overrides.enable, overrides.depth_enable);
  }
}

mod private {
  use crate::game::KeyValues;

  use dopamine_macros::virtual_method;

  use std::ffi::{c_char, c_void};

  #[repr(C)]
  pub struct Texture;

  impl Texture {
    #[virtual_method(index = 3)]
    fn actual_width(&self) -> i32;

    #[virtual_method(index = 4)]
    fn actual_height(&self) -> i32;
  }

  #[repr(C)]
  pub struct MaterialSystem;

  impl MaterialSystem {
    #[virtual_method(index = 70)]
    fn create_material<'a>(
      &self,
      name: *const c_char,
      kv: &KeyValues,
    ) -> Option<&'a super::Material>;

    #[virtual_method(index = 79)]
    fn find_texture(
      &self,
      name: *const c_char,
      group: *const c_char,
      complain: bool,
      additional_creation_flags: i32,
    ) -> Option<&super::Texture>;

    #[virtual_method(index = 85)]
    fn create_named_rt_ex(
      &self,
      name: *const c_char,
      width: i32,
      height: i32,
      size_mode: i32,
      image_format: i32,
      depth: i32,
      texture_flags: u32,
      rt_flags: u32,
    ) -> Option<&super::Texture>;
  }

  #[repr(C)]
  pub struct RenderContext;

  impl RenderContext {
    #[virtual_method(index = 12)]
    fn clear_buffers(&self, clear_color: bool, clear_depth: bool, clear_stencil: bool);

    #[virtual_method(index = 38)]
    fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32);

    #[virtual_method(index = 72)]
    fn clear_color_3u8(&self, r: u8, g: u8, b: u8);

    #[virtual_method(index = 74)]
    fn override_depth_enable(&self, enable: bool, depth_enable: bool);

    #[virtual_method(index = 103)]
    fn draw_screen_space_rect(
      &self,
      material: &super::Material,
      x: i32,
      y: i32,
      width: i32,
      height: i32,
      texture_x0: f32,
      texture_y0: f32,
      texture_x1: f32,
      texture_y1: f32,
      texture_width: i32,
      texture_height: i32,
      renderable: *mut c_void,
      dice_x: i32,
      dice_y: i32,
    );
  }
}
