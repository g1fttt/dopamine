use super::FeatureContext;
use crate::config::BunnyhopConfig;

use dopamine_sdk::game::UserCommand;

pub fn bunnyhop(ctx: FeatureContext<'_, '_, BunnyhopConfig>, cmd: &mut UserCommand) {
  if !ctx.config.enabled {
    return;
  }

  let should_bunnyhop = fastrand::u8(1..100) <= ctx.config.chance;
  if ctx.local_player.is_some_and(move |lp| !lp.is_on_ground() || !should_bunnyhop) {
    cmd.buttons &= !UserCommand::IN_JUMP;
  }
}
