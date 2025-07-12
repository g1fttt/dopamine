use dopamine_sdk::engine::{Model, ModelInfo};

use crate::app::App;

pub extern "fastcall" fn get_model(this: &ModelInfo, model_index: i32) -> Option<&Model> {
  App::with_mut(|app| {
    let model = (app.hooks.get_model.original)(this, model_index)?;

    let ctx = app.capture_context(&app.config.model_changer);

    let replacement = app.model_changer.on_get_model_call(ctx, model);

    if replacement.is_some() {
      return replacement;
    }

    Some(model)
  })
}
