use crate::config::BunnyhopConfig;
use crate::game::{Entity, UserCommand};

pub fn bunnyhop(config: &BunnyhopConfig, local_player: Option<&Entity>, cmd: &mut UserCommand) {
    if !config.enabled {
        return;
    }

    let should_bunnyhop = fastrand::u8(1..100) <= config.chance;
    if local_player.is_some_and(move |lp| !lp.is_on_ground() || !should_bunnyhop) {
        cmd.buttons &= !UserCommand::IN_JUMP;
    }
}
