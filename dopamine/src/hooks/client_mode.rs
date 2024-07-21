use crate::game::client::ClientMode;
use crate::game::render_view::ViewSetup;
use crate::game::UserCommand;

use crate::features::misc;
use crate::interfaces::Interfaces;
use crate::App;

type CreateMoveFn = extern "thiscall" fn(&ClientMode, f32, &mut UserCommand) -> bool;

pub extern "thiscall" fn create_move(
  this: &ClientMode,
  input_sample_frame_time: f32,
  cmd: &mut UserCommand,
) -> bool {
  App::with(move |app| {
    let original: CreateMoveFn = app.hooks.create_move.original();
    let result = original(this, input_sample_frame_time, cmd);

    misc::bunnyhop(&app.config.misc.bunnyhop, app.local_player, cmd);

    result
  })
}

type DoPostScreenSpaceEffects = extern "thiscall" fn(&ClientMode, &ViewSetup) -> bool;

pub extern "thiscall" fn do_post_screen_space_effects(this: &ClientMode, view: &ViewSetup) -> bool {
  App::with_mut(move |app| {
    let original: DoPostScreenSpaceEffects = app.hooks.do_post_screen_space_effects.original();
    let result = original(this, view);

    app
      .glow
      .draw(Interfaces::get(), &app.config.glow, app.local_player, view);

    result
  })
}
