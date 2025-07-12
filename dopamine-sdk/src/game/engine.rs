use crate::game::material_system::Material;
use crate::{cstr, cstr_path, rstr};

use dopamine_macros::virtual_method;

use std::ffi::{c_char, c_void};
use std::path::Path;

#[repr(C)]
pub struct Engine;

impl Engine {
  virtual_method!(pub fn local_player_index[12](&self) -> i32);
  virtual_method!(pub fn max_clients[21](&self) -> i32);
  virtual_method!(pub fn is_in_game[26](&self) -> bool);
}

// TODO: Use `IClientRenderable *pRenderable` instead of `int entity_index`
#[repr(C)]
pub struct ModelRenderInfo<'a> {
  pad1: [u8; 32],
  pub model: Option<&'a Model>,
  pad2: [u8; 28],
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
  virtual_method!(fn forced_material_override[1](&self, new_material: Option<&Material>)
    where (0: i32 /* NORMAL */));
}

#[repr(C)]
pub struct ModelInfo;

impl ModelInfo {
  #[inline]
  pub fn get_model_index(&self, model_path: impl AsRef<Path>) -> i32 {
    self.get_model_index_raw(cstr_path!(model_path.as_ref()))
  }

  #[inline]
  pub fn get_model_name(&self, model: Option<&Model>) -> Option<&'static str> {
    Some(unsafe { rstr!(self.get_model_name_raw(model?)) })
  }

  #[inline]
  pub fn find_or_load_model(&self, model_path: impl AsRef<Path>) -> Option<&'static Model> {
    self.find_or_load_model_raw(cstr_path!(model_path.as_ref()))
  }
}

impl ModelInfo {
  virtual_method!(fn get_model_index_raw[2](&self, model_path: *const c_char) -> i32);
  virtual_method!(fn get_model_name_raw[3](&self, model: &Model) -> *const c_char);
  // virtual_method!(fn get_studio_model_raw[28]
  //   (&self, model: &Model) -> Option<&'static mut StudioHeader>);
  virtual_method!(fn find_or_load_model_raw[39]
    (&self, model_path: *const c_char) -> Option<&'static Model>);
}

#[repr(C)]
pub struct Model;

#[repr(C)]
pub struct NetworkStringTable;

impl NetworkStringTable {
  #[inline]
  pub fn add_string(&self, is_server: bool, value: &str) {
    self.add_string_raw(is_server, cstr!(value));
  }
}

impl NetworkStringTable {
  virtual_method!(fn add_string_raw[8](&self, is_server: bool, value: *const c_char) -> i32
    where (-1: i32 /* length */, std::ptr::null(): *const c_void /* user_data */));
}

#[repr(C)]
pub struct NetworkStringTableContainer;

impl NetworkStringTableContainer {
  #[inline]
  pub fn find_table(&self, table_name: &str) -> Option<&NetworkStringTable> {
    self.find_table_raw(cstr!(table_name))
  }
}

impl NetworkStringTableContainer {
  virtual_method!(fn find_table_raw[3](
    &self, table_name: *const c_char) -> Option<&NetworkStringTable>);
}
