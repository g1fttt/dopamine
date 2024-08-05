use crate::App;

use dopamine_sdk::game::client::Client;
use dopamine_sdk::Interfaces;

pub extern "thiscall" fn level_init_post_entity(this: &Client) {
  App::with_mut(move |app| {
    (app.hooks.level_init_post_entity.original)(this);

    let interfaces = Interfaces::get();
    app.local_player =
      interfaces.entity_list.get_entity_by_index(interfaces.engine.local_player_index());
  });
}

pub extern "thiscall" fn level_shutdown(this: &Client) {
  App::with_mut(move |app| {
    (app.hooks.level_shutdown.original)(this);

    app.local_player = None;
  });
}
