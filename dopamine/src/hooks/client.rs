use crate::app::App;
use crate::entities;

use dopamine_sdk::client::Client;
use dopamine_sdk::ClassId;

pub extern "C" fn level_init_post_entity(this: &Client) {
  App::with_mut(move |app| {
    (app.hooks.level_init_post_entity.original)(this);

    app.player_resource =
      entities::iter().find(|&ent| ent.networkable().client_class().id == ClassId::PlayerResource);
  });
}

pub extern "C" fn level_shutdown(this: &Client) {
  App::with_mut(move |app| {
    (app.hooks.level_shutdown.original)(this);

    app.player_resource = None;
  });
}
