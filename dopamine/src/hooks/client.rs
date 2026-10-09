use crate::app::App;
use crate::entities;
use crate::state::GameState;

use dopamine_sdk::client::{Client, FrameStage};
use dopamine_sdk::interfaces::engine;
use dopamine_sdk::{ClassId, Hook};

pub extern "C" fn level_init_post_entity(this: &Client) {
  App::with_mut(move |app| {
    (app.hooks.level_init_post_entity.original())(this);

    app.player_resource =
      entities::iter().find(|&ent| ent.networkable().client_class().id == ClassId::PlayerResource);
  });
}

pub extern "C" fn level_shutdown(this: &Client) {
  App::with_mut(move |app| {
    (app.hooks.level_shutdown.original())(this);

    app.player_resource = None;
  });
}

pub extern "C" fn frame_stage_notify(this: &Client, current_stage: FrameStage) {
  App::with_mut(move |app| {
    (app.hooks.frame_stage_notify.original())(this, current_stage);

    if current_stage == FrameStage::RenderEnd && engine().is_in_game() {
      app.game_state.with_mut(GameState::collect);
    }
  });
}
