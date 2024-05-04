use crate::game::{Entity, UserCommand};

pub fn bunnyhop(local_player: Option<&Entity>, cmd: &mut UserCommand) {
    if local_player.is_some_and(|lp| !lp.is_on_ground()) {
        cmd.buttons &= !UserCommand::IN_JUMP;
    }
}
