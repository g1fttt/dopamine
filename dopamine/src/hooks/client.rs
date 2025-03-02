use crate::App;

use dopamine_sdk::client::Client;
use dopamine_sdk::utils::Interfaces;

pub extern "fastcall" fn level_init_post_entity(this: &Client) {
  App::with_mut(move |app| {
    (app.hooks.level_init_post_entity.original)(this);

    let interfaces = Interfaces::get();
    app.local_player =
      interfaces.entity_list.get_entity_by_index(interfaces.engine.local_player_index());
  });
}

pub extern "fastcall" fn level_shutdown(this: &Client) {
  App::with_mut(move |app| {
    (app.hooks.level_shutdown.original)(this);

    app.local_player = None;
  });
}
