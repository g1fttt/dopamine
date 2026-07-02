use crate::game::KeyValues;
use crate::{Color, cstr};

use crate::virtual_method;
use derive_builder::Builder;
use open_enum::open_enum;

use std::ffi::{c_char, c_void};

#[open_enum]
#[repr(C)]
pub enum MaterialFlag {
  IgnoreZ = 1 << 15,
  Wireframe = 1 << 28,
}

#[repr(C)]
pub struct Material;

impl Material {
  virtual_method!(pub fn set_flag[29](&self, flag: MaterialFlag, state: bool));
}

#[repr(C)]
pub struct Texture;

impl Texture {
  #[inline]
  pub fn dimensions(&self) -> (i32, i32) {
    (self.actual_width(), self.actual_height())
  }
}

impl Texture {
  virtual_method!(pub fn inc_ref_counter[10](&self));
}

impl Texture {
  virtual_method!(fn actual_width[3](&self) -> i32);
  virtual_method!(fn actual_height[4](&self) -> i32);
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
  pub fn create_named_rt(&self, name: &str, (width, height): (i32, i32)) -> Option<&Texture> {
    self.create_named_rt_ex(cstr!(name), width, height)
  }
}

impl MaterialSystem {
  virtual_method!(pub fn render_ctx[98](&self) -> &RenderContext);
}

impl MaterialSystem {
  virtual_method!(fn create_material_raw<'a>[70](&self, name: *const c_char, kv: &KeyValues) -> Option<&'a Material>);
  virtual_method!(fn find_texture_raw[79](&self, name: *const c_char, group: *const c_char) -> Option<&Texture> where (bool: true, i32: 0));
  virtual_method!(fn create_named_rt_ex[85]
    (&self, name: *const c_char, width: i32, height: i32) -> Option<&Texture>
      where (i32: 1, i32: 0, i32: 1, u32: 0x200C, u32: 1));
}

#[derive(Default, Clone, Copy)]
#[open_enum]
#[repr(C)]
pub enum StencilOp {
  #[default]
  Keep = 1,
  Replace = 3,
}

#[derive(Default, Clone, Copy)]
#[open_enum]
#[repr(C)]
pub enum StencilCmpFn {
  Equal = 3,
  #[default]
  Always = 8,
}

#[repr(C)]
pub struct RenderContext;

impl RenderContext {
  pub fn push_rt_and_set_viewport(&self, rt: &Texture, (width, height): (i32, i32)) {
    self.push_rt_and_viewport(rt);
    self.set_viewport(0, 0, width, height);
  }

  pub fn clear_color_3u8(&self, color: Color<u8>) {
    let color = color.mul_255_if_supports();
    self.clear_color_3u8_raw(color.r, color.g, color.b);
  }
}

impl RenderContext {
  virtual_method!(pub fn set_render_target[6](&self, texture: &Texture));
  virtual_method!(pub fn pop_rt_and_viewport[109](&self));
  virtual_method!(pub fn set_stencil_enable[117](&self, enable: bool));
  virtual_method!(pub fn set_stencil_fail_op[118](&self, op: StencilOp));
  virtual_method!(pub fn set_stencil_z_fail_op[119](&self, op: StencilOp));
  virtual_method!(pub fn set_stencil_pass_op[120](&self, op: StencilOp));
  virtual_method!(pub fn set_stencil_cmp_fn[121](&self, cmp_fn: StencilCmpFn));
  virtual_method!(pub fn set_stencil_ref_value[122](&self, ref_value: i32));
  virtual_method!(pub fn set_stencil_test_mask[123](&self, mask: u32));
  virtual_method!(pub fn set_stencil_write_mask[124](&self, mask: u32));
}

impl RenderContext {
  virtual_method!(fn clear_buffers[12](&self, clear_color: bool, clear_depth: bool, clear_stencil: bool));
  virtual_method!(fn set_viewport[38](&self, x: i32, y: i32, width: i32, height: i32));
  virtual_method!(fn clear_color_3u8_raw[72](&self, r: u8, g: u8, b: u8));
  virtual_method!(fn override_depth_enable[74](&self, enable: bool, depth_enable: bool));
  virtual_method!(fn draw_screen_space_rect[103]
    (&self, material: &Material, x: i32, y: i32, width: i32, height: i32,
    texture_x0: f32, texture_y0: f32, texture_x1: f32, texture_y1: f32,
    texture_width: i32, texture_height: i32)
      where (*mut c_void: std::ptr::null_mut(), i32: 1, i32: 1));
  virtual_method!(fn push_rt_and_viewport[107](&self, rt: &Texture));
}

#[derive(Builder)]
#[builder(pattern = "owned", derive(Clone))]
pub struct ScreenSpaceRect<'a> {
  material: &'a Material,
  pos: (i32, i32),
  dimensions: (i32, i32),
  texture_x0_y0: (f32, f32),
  texture_x1_y1: (f32, f32),
  texture_dimensions: (i32, i32),
}

impl ScreenSpaceRectBuilder<'_> {
  pub fn build_and_draw(self, render_ctx: &RenderContext) {
    let rect = self.build().unwrap();
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
    let buffers = self.build().unwrap();
    render_ctx.clear_buffers(buffers.clear_color, buffers.clear_depth, buffers.clear_stencil);
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
    let overrides = self.build().unwrap();
    render_ctx.override_depth_enable(overrides.enable, overrides.depth_enable);
  }
}
