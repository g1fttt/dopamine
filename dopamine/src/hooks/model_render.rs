use crate::App;

use dopamine_sdk::engine::{ModelRender, ModelRenderInfo};
use dopamine_sdk::utils::Interfaces;

use std::ffi::c_void;

pub extern "fastcall" fn draw_model_execute(
  this: &ModelRender,
  state: *mut c_void,
  info: &ModelRenderInfo,
  custom_bone_to_world: *mut c_void,
) {
  App::with_mut(move |app| {
    let original = app.hooks.draw_model_execute.original;
    let original = move || original(this, state, info, custom_bone_to_world);

    let interfaces = Interfaces::get();

    if interfaces.studio_render.is_material_overrided() {
      return original();
    }

    let ctx = app.capture_context(&app.config.model_changer);

    if app.model_changer.should_remove_sleeves(ctx, info.model) {
      return;
    }

    let entity = interfaces.entity_list.get_entity_by_index(info.entity_index);

    // FIXME: If enabled at least one ignore-z layer along with a glow
    //        then ignore-z chams shall be visible even if model isn't occluded
    app.chams.draw(app.capture_context(&app.config.chams), &original, entity);

    if !app.chams.applied() {
      original();
    }

    interfaces.model_render.reset_material();
  });
}
