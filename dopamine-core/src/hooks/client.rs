use crate::app::App;

use std::ffi::c_void;

type LevelInitPostEntityFn = extern "thiscall" fn(*mut c_void);

pub extern "thiscall" fn level_init_post_entity(this: *mut c_void) {
    App::with_mut(move |app| {
        let original: LevelInitPostEntityFn = app.hooks.level_init_post_entity.original();
        original(this);

        app.local_player = app
            .interfaces
            .entity_list
            .get_entity_by_index(app.interfaces.engine.local_player_index());
    });
}

type LevelShutdownFn = extern "thiscall" fn(*mut c_void);

pub extern "thiscall" fn level_shutdown(this: *mut c_void) {
    App::with_mut(move |app| {
        let original: LevelShutdownFn = app.hooks.level_shutdown.original();
        original(this);

        app.local_player = None;
    });
}
