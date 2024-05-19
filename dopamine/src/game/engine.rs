use super::material_system::Material;
use super::Entity;

use dopamine_macros::virtual_method;

#[repr(C)]
pub struct Engine;

impl Engine {
    #[virtual_method(index = 12)]
    fn local_player_index(&self) -> i32;

    #[virtual_method(index = 21)]
    fn max_clients(&self) -> i32;

    #[virtual_method(index = 26)]
    fn is_in_game(&self) -> bool;
}

#[repr(C)]
pub struct ModelRenderInfo<'a> {
    pad: [u8; 24],
    pub renderable: &'a Entity,
}

#[repr(C)]
pub struct ModelRender;

impl ModelRender {
    pub fn forced_material_override(&self, new_material: Option<&Material>) {
        self.forced_material_override_private(new_material, 0 /* Normal */);
    }
}

impl ModelRender {
    #[virtual_method(index = 1, private)]
    fn forced_material_override_private(&self, new_material: Option<&Material>, override_kind: i32);
}
