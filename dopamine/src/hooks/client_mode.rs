use crate::features::{misc, visuals};
use crate::App;

use dopamine_sdk::game::client::ClientMode;
use dopamine_sdk::game::render_view::ViewSetup;
use dopamine_sdk::game::UserCommand;

type OverrideViewFn = extern "thiscall" fn(&ClientMode, &mut ViewSetup);

pub extern "thiscall" fn override_view(this: &ClientMode, view: &mut ViewSetup) {
  App::with(move |app| {
    let original: OverrideViewFn = app.hooks.override_view.original();
    original(this, view);

    visuals::add_fov(&app.config.visuals.add_fov, view);
  })
}

type CreateMoveFn = extern "thiscall" fn(&ClientMode, f32, &mut UserCommand) -> bool;

pub extern "thiscall" fn create_move(
  this: &ClientMode,
  input_sample_frame_time: f32,
  cmd: &mut UserCommand,
) -> bool {
  App::with(move |app| {
    let original: CreateMoveFn = app.hooks.create_move.original();
    let result = original(this, input_sample_frame_time, cmd);

    misc::bunnyhop(app.capture_context(&app.config.misc.bunnyhop), cmd);

    result
  })
}

type DoPostScreenSpaceEffects = extern "thiscall" fn(&ClientMode, &ViewSetup) -> bool;

pub extern "thiscall" fn do_post_screen_space_effects(this: &ClientMode, view: &ViewSetup) -> bool {
  App::with_mut(move |app| {
    let original: DoPostScreenSpaceEffects = app.hooks.do_post_screen_space_effects.original();
    let result = original(this, view);

    app.glow.draw(app.capture_context(&app.config.glow), view);

    result
  })
}
