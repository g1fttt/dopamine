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

/*
struct ModelRenderInfo {
    Vector3 origin; // 0
    QAngle angles; // 12
    void* renderable; // 24
    const void* model; // 28
    const Matrix3x4* modelToWorld; // 32
    const Matrix3x4* lightningOffset; // 36
    const Vector3* lightningOrigin; // 40
    int flags; // 44
    int entityIndex;
    int skin;
    int body;
    int hitboxset;
    ModelRenderInstance instance;
};
*/

#[repr(C)]
pub struct ModelRenderInfo {
    pad: [u8; 44],
    pub entity_index: i32,
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
