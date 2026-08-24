use crate::game::material_system::Material;
use crate::{cstr, rstr, virtual_method};

use std::ffi::c_char;
use std::ptr;

#[repr(C)]
pub struct Engine;

impl Engine {
  virtual_method!(pub fn local_player_index[12](&self) -> i32);
  virtual_method!(pub fn max_clients[21](&self) -> i32);
  virtual_method!(pub fn is_in_game[26](&self) -> bool);
}

// TODO: Use `IClientRenderable *pRenderable` instead of `int entity_index`
#[repr(C)]
pub struct ModelRenderInfo {
  pad: [u8; 68],
  pub entity_index: i32,
}

#[repr(C)]
pub struct ModelRender;

impl ModelRender {
  pub fn override_material(&self, new_material: &Material) {
    self.forced_material_override(Some(new_material));
  }

  pub fn reset_material(&self) {
    self.forced_material_override(None);
  }
}

impl ModelRender {
  virtual_method!(fn forced_material_override[1](&self, new_material: Option<&Material>)
    where (i32: 0 /* NORMAL */));
}

#[repr(C)]
pub struct GameEvent;

impl GameEvent {
  pub fn name<'a>(&self) -> &'a str {
    unsafe { rstr!(self.name_raw()) }
  }

  pub fn get_int(&self, key_name: &str) -> Option<i32> {
    match self.get_int_raw(cstr!(key_name)) {
      -1 => None,
      n => Some(n),
    }
  }

  pub fn get_string<'a>(&self, key_name: &str) -> Option<&'a str> {
    let raw = self.get_string_raw(cstr!(key_name));
    if raw.is_null() { None } else { Some(unsafe { rstr!(raw) }) }
  }
}

impl GameEvent {
  virtual_method!(fn name_raw[1](&self) -> *const c_char);
  virtual_method!(fn get_int_raw[6](&self, key_name: *const c_char) -> i32
    where (i32: -1 /* defaultValue */));
  virtual_method!(fn get_string_raw[8](&self, key_name: *const c_char) -> *const c_char
    where (*const c_char: ptr::null() /* defaultValue */));
}

#[repr(C)]
pub struct GameEventManager;
