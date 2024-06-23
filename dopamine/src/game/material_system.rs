use super::KeyValues;
use crate::cstr;
use crate::utils::Color;

use derive_builder::Builder;
use dopamine_macros::virtual_method;

use std::ffi::{c_char, c_void};
use std::ptr;

pub type Vec2<T> = (T, T);

#[repr(C)]
pub enum MaterialFlag {
  IgnoreZ = 1 << 15,
}

#[repr(C)]
pub struct Material;

impl Material {
  virtual_method!(pub fn set_flag(&self, flag: MaterialFlag, state: bool) [29]);
}

#[repr(C)]
pub struct Texture;

impl Texture {
  pub fn dimensions(&self) -> (i32, i32) {
    (self.actual_width(), self.actual_height())
  }
}

impl Texture {
  virtual_method!(pub fn actual_width(&self) -> i32 [3]);
  virtual_method!(pub fn actual_height(&self) -> i32 [4]);
  virtual_method!(pub fn inc_ref_counter(&self) [10]);
}

#[repr(C)]
pub struct MaterialSystem;

impl MaterialSystem {
  #[inline]
  pub fn create_material(&self, name: &str, kv: &KeyValues) -> Option<&Material> {
    self.create_material_raw(cstr!(name), kv)
  }

  #[inline]
  pub fn find_texture(&self, name: &str, group: &str) -> Option<&Texture> {
    self.find_texture_raw(cstr!(name), cstr!(group))
  }

  #[inline]
  pub fn create_named_rt(&self, name: &str, (width, height): Vec2<i32>) -> Option<&Texture> {
    self.create_named_rt_ex(cstr!(name), width, height)
  }
}

impl MaterialSystem {
  virtual_method!(fn create_material_raw<'a>(&self, name: *const c_char, kv: &KeyValues) -> Option<&'a Material> [70]);
  virtual_method!(fn find_texture_raw(&self, name: *const c_char, group: *const c_char) -> Option<&Texture> [79] => (true: bool, 0: i32));
  virtual_method!(fn create_named_rt_ex(&self, name: *const c_char, width: i32, height: i32) -> Option<&Texture> [85] => (1: i32, 0: i32, 1: i32, 0x200C: u32, 1: u32));
}

impl MaterialSystem {
  virtual_method!(pub fn render_ctx(&self) -> &RenderContext [98]);
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

#[repr(C)]
pub struct RenderContext;

impl RenderContext {
  pub fn push_rt_and_set_viewport(&self, rt: &Texture, (width, height): Vec2<i32>) {
    self.push_rt_and_viewport(rt);
    self.set_viewport(0, 0, width, height);
  }

  pub fn clear_color_3u8(&self, color: Color<u8>) {
    let color = color.mul_255_if_supports();
    self.clear_color_3u8_raw(color.r, color.g, color.b);
  }
}

impl RenderContext {
  virtual_method!(pub fn set_render_target(&self, texture: &Texture) [6]);
  virtual_method!(pub fn pop_rt_and_viewport(&self) [109]);
  virtual_method!(pub fn set_stencil_enable(&self, enable: bool) [117]);
  virtual_method!(pub fn set_stencil_fail_op(&self, op: StencilOp) [118]);
  virtual_method!(pub fn set_stencil_z_fail_op(&self, op: StencilOp) [119]);
  virtual_method!(pub fn set_stencil_pass_op(&self, op: StencilOp) [120]);
  virtual_method!(pub fn set_stencil_cmp_fn(&self, cmp_fn: StencilCmpFn) [121]);
  virtual_method!(pub fn set_stencil_ref_value(&self, ref_value: i32) [122]);
  virtual_method!(pub fn set_stencil_test_mask(&self, mask: u32) [123]);
  virtual_method!(pub fn set_stencil_write_mask(&self, mask: u32) [124]);
}

impl RenderContext {
  virtual_method!(fn clear_buffers(&self, clear_color: bool, clear_depth: bool, clear_stencil: bool) [12]);
  virtual_method!(fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32) [38]);
  virtual_method!(fn clear_color_3u8_raw(&self, r: u8, g: u8, b: u8) [72]);
  virtual_method!(fn override_depth_enable(&self, enable: bool, depth_enable: bool) [74]);
  virtual_method!(fn draw_screen_space_rect(
    &self,
    material: &Material,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    texture_x0: f32,
    texture_y0: f32,
    texture_x1: f32,
    texture_y1: f32,
    texture_width: i32,
    texture_height: i32
  ) [103] => (ptr::null_mut(): *mut c_void, 1: i32, 1: i32));
  virtual_method!(fn push_rt_and_viewport(&self, rt: &Texture) [107]);
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
    render_ctx.draw_screen_space_rect(
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
    render_ctx.clear_buffers(
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
    render_ctx.override_depth_enable(overrides.enable, overrides.depth_enable);
  }
}
