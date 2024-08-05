use crate::features::{misc, visuals};
use crate::App;

use dopamine_sdk::game::client::ClientMode;
use dopamine_sdk::game::render_view::ViewSetup;
use dopamine_sdk::game::UserCommand;

pub extern "thiscall" fn override_view(this: &ClientMode, view: &mut ViewSetup) {
  App::with(move |app| {
    (app.hooks.override_view.original)(this, view);

    visuals::add_fov(&app.config.visuals.add_fov, view);
  })
}

pub extern "thiscall" fn create_move(
  this: &ClientMode,
  input_sample_frame_time: f32,
  cmd: &mut UserCommand,
) -> bool {
  App::with(move |app| {
    let result = (app.hooks.create_move.original)(this, input_sample_frame_time, cmd);
    {
      misc::bunnyhop(app.capture_context(&app.config.misc.bunnyhop), cmd);
    }
    result
  })
}

// char __stdcall ClientModeShared::DoPostScreenSpaceEffects(int a1)
// {
//   return 1;
// }
pub extern "thiscall" fn do_post_screen_space_effects(_: &ClientMode, view: &ViewSetup) -> bool {
  App::with_mut(move |app| app.glow.draw(app.capture_context(&app.config.glow), view));

  true
}
