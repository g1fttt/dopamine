use crate::App;

use dopamine_sdk::game::engine::{ModelRender, ModelRenderInfo};
use dopamine_sdk::Interfaces;

use std::ffi::c_void;

type DrawModelExecuteFn =
  extern "thiscall" fn(&ModelRender, *mut c_void, &ModelRenderInfo, *mut c_void);

pub extern "thiscall" fn draw_model_execute(
  this: &ModelRender,
  state: *mut c_void,
  info: &ModelRenderInfo,
  custom_bone_to_world: *mut c_void,
) {
  App::with_mut(move |app| {
    let original: DrawModelExecuteFn = app.hooks.draw_model_execute.original();
    let original = move || original(this, state, info, custom_bone_to_world);

    let interfaces = Interfaces::get();

    if interfaces.studio_render.is_material_overrided() {
      return original();
    }

    // FIXME: Chams also applies onto world-model weapons
    app.chams.draw(app.capture_context(&app.config.chams), &original, info);

    if !app.chams.applied() {
      original();
    }
    interfaces.model_render.reset_material();
  });
}
