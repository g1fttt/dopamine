#include "hack.h"

#include <game/engine.h>
#include <game/entity.h>

#include <entity_listener.h>

namespace glow
{
  void Hack::manage_players(ObjectManager &object_manager,
                            game::PlayerEntity *local_player) const {
    if (!local_player) {
      return;
    }

    for (const auto player: app->entity_listener.players) {
      const auto is_enemy = player->team() != local_player->team();

      if (config.enemies.enabled && is_enemy) {
        object_manager.update_object_by_entity(player, config.enemies.color);
      } else if (config.allies.enabled && !is_enemy) {
        object_manager.update_object_by_entity(player, config.allies.color);
      }
    }
  }
}
