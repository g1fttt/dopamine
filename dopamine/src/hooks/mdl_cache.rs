use crate::app::App;

use dopamine_sdk::data_cache::{MdlCache, ModelHandle};
use dopamine_sdk::{StudioHardwareData, StudioHeader};

pub extern "fastcall" fn get_studio_header(
  this: &MdlCache,
  handle: ModelHandle,
) -> Option<&mut StudioHeader> {
  App::with_mut(|app| {
    let original = |h| (app.hooks.get_studio_header.original)(this, h);
    let result = original(handle);

    let ctx = app.capture_context(&app.config.model_changer);

    let replacement = app.model_changer.on_studio_call(ctx, handle, original);

    if replacement.is_some() {
      return replacement;
    }

    result
  })
}

pub extern "fastcall" fn get_hardware_data(
  this: &MdlCache,
  handle: ModelHandle,
) -> Option<&mut StudioHardwareData> {
  App::with_mut(|app| {
    let original = |h| (app.hooks.get_hardware_data.original)(this, h);
    let result = original(handle);

    let ctx = app.capture_context(&app.config.model_changer);

    let replacement = app.model_changer.on_studio_call(ctx, handle, original);

    if replacement.is_some() {
      return replacement;
    }

    result
  })
}
