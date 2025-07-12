use crate::app::App;
use crate::entities;

use dopamine_sdk::ClassId;
use dopamine_sdk::client::{Client, FrameStage};
use dopamine_sdk::utils::Interfaces;

pub extern "fastcall" fn level_init_post_entity(this: &Client) {
  App::with_mut(move |app| {
    (app.hooks.level_init_post_entity.original)(this);

    let interfaces = Interfaces::get();

    app.local_player =
      interfaces.entity_list.get_entity_by_index(interfaces.engine.local_player_index());
    app.player_resource =
      entities::iter().find(|&ent| ent.networkable().client_class().id == ClassId::PlayerResource);
  });
}

pub extern "fastcall" fn level_shutdown(this: &Client) {
  App::with_mut(move |app| {
    (app.hooks.level_shutdown.original)(this);

    app.local_player = None;
    app.player_resource = None;

    app.model_changer.destroy_entities();
  });
}

pub extern "fastcall" fn frame_stage_notify(this: &Client, stage: FrameStage) {
  App::with_mut(move |app| {
    if stage == FrameStage::RenderStart {
      app.model_changer.on_fsn_call(app.capture_context(&app.config.model_changer));
    }
    (app.hooks.frame_stage_notify.original)(this, stage);
  });
}
