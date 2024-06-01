use super::material_system::Material;

use dopamine_macros::virtual_method;

#[repr(C)]
pub struct Engine;

impl Engine {
  #[virtual_method(index = 12)]
  fn local_player_index(&self) -> i32;

  #[virtual_method(index = 21)]
  fn max_clients(&self) -> i32;
}

#[repr(C)]
pub struct ModelRenderInfo {
  pad: [u8; 44],
  pub entity_index: i32,
}

#[repr(transparent)]
pub struct ModelRender(raw::ModelRender);

impl ModelRender {
  pub fn forced_material_override(&self, new_material: Option<&Material>) {
    self
      .as_ref()
      .forced_material_override(new_material, 0 /* Normal */);
  }
}

impl AsRef<raw::ModelRender> for ModelRender {
  fn as_ref(&self) -> &raw::ModelRender {
    &self.0
  }
}

mod raw {
  use crate::game::material_system::Material;

  use dopamine_macros::virtual_method;

  #[repr(C)]
  pub struct ModelRender;

  impl ModelRender {
    #[virtual_method(index = 1)]
    fn forced_material_override(&self, new_material: Option<&Material>, override_kind: i32);
  }
}
