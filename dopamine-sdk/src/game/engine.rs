use crate::game::material_system::Material;
use crate::virtual_method;

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
