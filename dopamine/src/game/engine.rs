use super::material_system::Material;

use dopamine_macros::virtual_method;

use std::ffi::{c_char, CStr};

#[repr(C)]
pub struct Engine;

impl Engine {
  virtual_method!(pub fn local_player_index[12](&self) -> i32);
  virtual_method!(pub fn max_clients[21](&self) -> i32);
  virtual_method!(pub fn is_in_game[26](&self) -> bool);
}

#[repr(C)]
pub struct ModelRenderInfo<'a> {
  pad1: [u8; 28],
  pub model: &'a Model,
  pad2: [u8; 16],
  pub entity_index: i32,
}

#[repr(C)]
pub struct ModelRender;

impl ModelRender {
  #[inline]
  pub fn override_material(&self, new_material: &Material) {
    self.forced_material_override(Some(new_material));
  }

  #[inline]
  pub fn reset_material(&self) {
    self.forced_material_override(None);
  }
}

impl ModelRender {
  virtual_method!(fn forced_material_override[1](&self, new_material: Option<&Material>) where (0: i32 /* NORMAL */));
}

#[repr(C)]
pub struct Model;

#[repr(C)]
pub struct ModelInfo;

impl ModelInfo {
  pub fn model_name(&self, model: &Model) -> &str {
    unsafe { CStr::from_ptr(self.model_name_raw(model)) }.to_str().unwrap()
  }
}

impl ModelInfo {
  virtual_method!(fn model_name_raw[3](&self, model: &Model) -> *const c_char);
}
