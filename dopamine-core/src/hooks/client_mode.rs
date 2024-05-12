use crate::game::client::ClientMode;
use crate::game::render_view::ViewSetup;
use crate::game::UserCommand;

use crate::app::App;
use crate::features::{glow, misc};

type CreateMoveFn = extern "thiscall" fn(&mut ClientMode, f32, &mut UserCommand) -> bool;

pub extern "thiscall" fn create_move(
    this: &mut ClientMode,
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

type DoPostScreenSpaceEffects = extern "thiscall" fn(&mut ClientMode, &ViewSetup) -> bool;

pub extern "thiscall" fn do_post_screen_space_effects(
    this: &mut ClientMode,
    view: &ViewSetup,
) -> bool {
    App::with_mut(move |app| {
        let original: DoPostScreenSpaceEffects = app.hooks.do_post_screen_space_effects.original();
        let result = original(this, view);

        if app.interfaces.engine.is_in_game() {
            let glow_object_manager = &mut app.glow_object_manager;

            glow::manage_players(
                &app.config.glow,
                &app.interfaces,
                glow_object_manager,
                app.local_player,
            );

            glow_object_manager.draw_glow_effects(&app.interfaces, view);
            glow_object_manager.clear_objects();
        }
        result
    })
}
