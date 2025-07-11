use crate::App;
use crate::features::{misc, visuals};

use dopamine_sdk::UserCommand;
use dopamine_sdk::client::ClientMode;
use dopamine_sdk::render_view::ViewSetup;

pub extern "fastcall" fn override_view(this: &ClientMode, view: &mut ViewSetup) {
  App::with_mut(move |app| {
    (app.hooks.override_view.original)(this, view);

    visuals::add_fov(&app.config.visuals.add_fov, view);
  })
}

pub extern "fastcall" fn create_move(
  this: &ClientMode,
  input_sample_frame_time: f32,
  cmd: &mut UserCommand,
) -> bool {
  App::with_mut(move |app| {
    let result = (app.hooks.create_move.original)(this, input_sample_frame_time, cmd);
    {
      misc::bunnyhop(app.capture_context(&app.config.misc.bunnyhop), cmd);
    }
    result
  })
}

pub extern "fastcall" fn should_draw_crosshair(_this: &ClientMode) -> bool {
  App::with_mut(move |app| !app.config.visuals.better_crosshair.enabled)
}

pub extern "fastcall" fn do_post_screen_space_effects(
  _this: &ClientMode,
  view: &ViewSetup,
) -> bool {
  App::with_mut(move |app| app.glow.draw(app.capture_context(&app.config.glow), view));

  true
}
