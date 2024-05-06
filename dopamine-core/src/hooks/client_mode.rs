use crate::app::App;
use crate::game::UserCommand;
use crate::hacks::misc;

use std::ffi::c_void;

type CreateMoveFn = extern "thiscall" fn(*mut c_void, f32, &mut UserCommand) -> bool;

pub extern "thiscall" fn create_move(
    this: *mut c_void,
    input_sample_frame_time: f32,
    cmd: &mut UserCommand,
) -> bool {
    App::with(move |app| {
        let original: &CreateMoveFn = app.hooks.create_move.original();
        let result = original(this, input_sample_frame_time, cmd);

        misc::bunnyhop(&app.config.misc.bunnyhop, app.local_player, cmd);

        result
    })
}
