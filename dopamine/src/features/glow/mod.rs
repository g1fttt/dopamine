mod object_manager;

pub use object_manager::*;

use crate::config::GlowGroupConfig;
use crate::game::Entity;
use crate::interfaces::Interfaces;

use std::mem;

pub fn manage_players<'a>(
    config: &'a GlowGroupConfig,
    interfaces: &'a Interfaces,
    object_manager: &mut GlowObjectManager,
    local_player: Option<&Entity>,
) {
    let Some(local_player) = local_player else {
        return;
    };

    for i in 1..interfaces.engine.max_clients() {
        let Some(entity) = interfaces.entity_list.get_entity_by_index(i) else {
            continue;
        };

        if entity.is_local_player() || object_manager.has_glow_effect(entity) {
            continue;
        }

        let is_enemy = entity.team() != local_player.team();

        let object_manager: &mut GlowObjectManager<'a> =
            unsafe { mem::transmute_copy(&object_manager) };

        if config.enemies.enabled && is_enemy {
            object_manager.register_object((entity, &config.enemies).into());
        } else if config.allies.enabled && !is_enemy {
            object_manager.register_object((entity, &config.allies).into());
        }
    }
}
