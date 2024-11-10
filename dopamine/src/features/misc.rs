use super::FeatureContext;

use dopamine_sdk::UserCommand;
use educe::Educe;
use serde::{Deserialize, Serialize};

pub fn bunnyhop(ctx: FeatureContext<'_, '_, BunnyhopConfig>, cmd: &mut UserCommand) {
  if !ctx.config.enabled {
    return;
  }

  let should_bunnyhop = fastrand::u8(1..100) <= ctx.config.chance;
  if ctx.local_player.is_some_and(move |lp| !lp.is_on_ground() || !should_bunnyhop) {
    cmd.buttons &= !UserCommand::IN_JUMP;
  }
}

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct BunnyhopConfig {
  pub enabled: bool,
  #[educe(Default = 100)]
  pub chance: u8,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MiscConfig {
  pub bunnyhop: BunnyhopConfig,
}
