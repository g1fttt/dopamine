use dopamine_sdk::{Entity, UserCommand};

use educe::Educe;
use serde::{Deserialize, Serialize};

pub fn bunnyhop(config: &BunnyhopConfig, cmd: &mut UserCommand) {
  if !config.enabled {
    return;
  }

  let should_bunnyhop = fastrand::i32(1..100) <= config.chance;
  if Entity::local_player().is_some_and(move |lp| !lp.is_on_ground() || !should_bunnyhop) {
    cmd.buttons &= !UserCommand::IN_JUMP;
  }
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
  pub disable_model_occlusion: bool,
}
