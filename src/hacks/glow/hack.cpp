#include "hack.h"

#include <game/engine.h>
#include <game/entity.h>
#include <game/entity_list.h>

#include <app.h>

namespace glow
{
  void Hack::manage_players(const core::Interfaces &interfaces,
                            ObjectManager &object_manager,
                            game::PlayerEntity *local_player) const {
    if (!local_player) {
      return;
    }

    for (int32_t i = 1; i < interfaces.engine->max_clients(); i += 1) {
      const auto entity = interfaces.entity_list->get_entity_by_index(i);
      if (!entity) {
        continue;
      }

      const auto player = entity->as<game::PlayerEntity>();
      if (player->is_local_player() || object_manager.has_glow_effect(player)) {
        continue;
      }

      const auto is_enemy = player->team() != local_player->team();

      if (config.enemies.enabled && is_enemy) {
        object_manager.register_object({player, config.enemies.color});
      } else if (config.allies.enabled && !is_enemy) {
        object_manager.register_object({player, config.allies.color});
      }
    }
  }
}
