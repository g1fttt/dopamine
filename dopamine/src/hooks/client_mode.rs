use crate::game::client::ClientMode;
use crate::game::render_view::ViewSetup;
use crate::game::UserCommand;

use crate::features::misc;
use crate::features::shared::RenderableObject;

use crate::app::App;

type CreateMoveFn = extern "thiscall" fn(&ClientMode, f32, &mut UserCommand) -> bool;

pub(super) extern "thiscall" fn create_move(
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

pub(super) extern "thiscall" fn do_post_screen_space_effects(
    this: &ClientMode,
    view: &ViewSetup,
) -> bool {
    App::with_mut(move |app| {
        let original: DoPostScreenSpaceEffects = app.hooks.do_post_screen_space_effects.original();
        let result = original(this, view);

        let interfaces = &app.interfaces;

        // TODO: Separate struct for this
        let players_iter = (1..interfaces.engine.max_clients())
            .filter_map(move |i| interfaces.entity_list.get_entity_by_index(i))
            .filter(|ent| !ent.is_local_player());

        let glow_objects: Vec<_> = players_iter.map(RenderableObject::new).collect();

        if interfaces.engine.is_in_game() {
            app.glow_object_manager
                .draw_glow_effects(&glow_objects, app, view);
        }
        result
    })
}
