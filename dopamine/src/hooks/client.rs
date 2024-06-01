use crate::app::App;
use crate::game::client::Client;
use crate::interfaces::Interfaces;

type LevelInitPostEntityFn = extern "thiscall" fn(&Client);

pub extern "thiscall" fn level_init_post_entity(this: &Client) {
  App::with_mut(move |app| {
    let original: LevelInitPostEntityFn = app.hooks.level_init_post_entity.original();
    original(this);

    let interfaces = Interfaces::get();
    app.local_player = interfaces
      .entity_list
      .get_entity_by_index(interfaces.engine.local_player_index());
  });
}

type LevelShutdownFn = extern "thiscall" fn(&Client);

pub extern "thiscall" fn level_shutdown(this: &Client) {
  App::with_mut(move |app| {
    let original: LevelShutdownFn = app.hooks.level_shutdown.original();
    original(this);

    app.local_player = None;
  });
}
