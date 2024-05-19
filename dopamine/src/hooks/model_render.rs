use crate::game::engine::{ModelRender, ModelRenderInfo};
use crate::App;

use std::ffi::c_void;

type DrawModelExecuteFn =
    extern "thiscall" fn(&ModelRender, *mut c_void, &ModelRenderInfo, *mut c_void);

pub(super) extern "thiscall" fn draw_model_execute(
    this: &ModelRender,
    state: *mut c_void,
    info: &ModelRenderInfo,
    custom_bone_to_world: *mut c_void,
) {
    App::with(move |app| {
        let original: DrawModelExecuteFn = app.hooks.draw_model_execute.original();
        original(this, state, info, custom_bone_to_world);
    });
}
