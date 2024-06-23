use super::material_system::Material;

use dopamine_macros::virtual_method;

#[repr(C)]
pub struct Engine;

impl Engine {
  virtual_method!(pub fn local_player_index(&self) -> i32 [12]);
  virtual_method!(pub fn max_clients(&self) -> i32 [21]);
  virtual_method!(pub fn is_in_game(&self) -> bool [26]);
}

#[repr(C)]
pub struct ModelRenderInfo {
  pad: [u8; 44],
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
  virtual_method!(fn forced_material_override(&self, new_material: Option<&Material>) [1] => (10: i32 /* Normal */));
}
