use crate::app::App;

use dopamine_sdk::Hook;
use dopamine_sdk::engine::{ModelRender, ModelRenderInfo};
use dopamine_sdk::interfaces::{entity_list, model_render, studio_render};

use std::ffi::c_void;

pub extern "C" fn draw_model_execute(
  this: &ModelRender,
  state: *mut c_void,
  info: &ModelRenderInfo,
  custom_bone_to_world: *mut c_void,
) {
  App::with_mut(move |app| {
    let original = app.hooks.draw_model_execute.original();
    let original = move || original(this, state, info, custom_bone_to_world);

    if studio_render().is_material_overrided() || app.glow.is_in_drawing_process() {
      return original();
    }
    let entity = entity_list().get_entity_by_index(info.entity_index);

    // FIXME: If enabled at least one ignore-z layer along with a glow
    //        then ignore-z chams shall be visible even if model isn't occluded
    app.chams.draw(&app.config.chams, &original, entity);

    if !app.chams.applied() {
      original();
    }
    model_render().reset_material();
  });
}
