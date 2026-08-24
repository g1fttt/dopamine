use crate::app::App;
use crate::features::misc::{self, GameState};

use presenceforge::DiscordIpcClient;

use dopamine_sdk::Hook;
use dopamine_sdk::engine::{GameEvent, GameEventManager};

pub extern "C" fn fire_event_client_side(this: &GameEventManager, event: &GameEvent) -> bool {
  App::with_mut(move |app| {
    let original = app.hooks.fire_event_client_side.original();

    let client = match app
      .discord_ipc_client
      .get_mut_or_try_init(|| DiscordIpcClient::new(env!("DOPAMINE_DISCORD_RICH_PRESENCE")))
    {
      Ok(client) => client,
      Err(err) => {
        log::error!("Failed to initialize Discord Rich Presence: {}", err);
        return original(this, event);
      }
    };

    let new_state = match event.name() {
      "client_disconnect" => GameState::InMenu,
      "server_spawn" => GameState::OnServer {
        host_name: event.get_string("hostname").unwrap(),
        map_name: event.get_string("mapname").unwrap(),
        address: event.get_string("address").unwrap(),
      },
      _ => return original(this, event),
    };

    if let Err(err) =
      misc::discord_rich_presence(app.config.misc.enable_discord_rich_presence, new_state, client)
    {
      log::error!("Failed to update Discord Rich Presence: {}", err);
    }
    original(this, event)
  })
}
