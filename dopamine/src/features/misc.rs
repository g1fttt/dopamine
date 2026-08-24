use dopamine_sdk::{Entity, UserCommand};
use educe::Educe;
use presenceforge::{ActivityBuilder, DiscordIpcClient, DiscordIpcError};
use serde::{Deserialize, Serialize};

use std::borrow::Cow;

pub fn bunnyhop(config: &BunnyhopConfig, cmd: &mut UserCommand) {
  if !config.enabled {
    return;
  }

  let should_bunnyhop = fastrand::i32(1..100) <= config.chance;
  if Entity::local_player().is_some_and(move |lp| !lp.is_on_ground() || !should_bunnyhop) {
    cmd.buttons &= !UserCommand::IN_JUMP;
  }
}

pub fn discord_rich_presence(
  enabled: bool,
  new_state: GameState,
  client: &mut DiscordIpcClient,
) -> presenceforge::Result<()> {
  if !enabled {
    if client.is_connected() {
      client.clear_activity()?;
    }

    return Ok(());
  }

  if !client.is_connected()
    && let Err(err) = client.connect()
    && matches!(err, DiscordIpcError::ConnectionFailed(_))
  {
    client.reconnect()?;
  }

  let activity = match new_state {
    GameState::InMenu => ActivityBuilder::new().state("In main menu"),
    GameState::OnServer { host_name, map_name, address } => {
      let is_local_server = address == "loopback";
      let state = match is_local_server {
        true => Cow::Borrowed("Playing on a local server"),
        false => Cow::Owned(format!("Playing on a server: {}", map_name)),
      };

      ActivityBuilder::new().state(state).details(host_name)
    }
  };

  client.set_activity(&activity.build())
}

pub enum GameState<'a> {
  InMenu,
  OnServer { host_name: &'a str, map_name: &'a str, address: &'a str },
}

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct BunnyhopConfig {
  pub enabled: bool,
  #[educe(Default = 100)]
  pub chance: i32,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MiscConfig {
  pub bunnyhop: BunnyhopConfig,
  pub enable_discord_rich_presence: bool,
}
