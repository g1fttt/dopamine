#include "hack.h"

#include <game/engine.h>
#include <game/entity.h>
#include <game/entity_list.h>

#include <interfaces.h>

#include "object_manager.h"

namespace glow {
  void Hack::manage_entities(const core::Interfaces &interfaces,
                             ObjectManager &object_manager,
                             game::PlayerEntity *local_player) const {
    for (int32_t i = 1; i < interfaces.engine->max_clients(); i += 1) {
      const auto entity = interfaces.entity_list->get_entity_by_index(i);
      if (!entity || !entity->is_player() || entity == local_player) {
        continue;
      }

      const auto player = entity->as<game::PlayerEntity>();
      const auto is_enemy = player->team() != local_player->team();

      if (!object_manager.has_glow_effect(entity)) {
        object_manager.register_entity(entity);
      }

      if (config.enemies.enabled && is_enemy) {
        object_manager.update_object_by_entity(player, config.enemies.color);
      } else if (config.allies.enabled && !is_enemy) {
        object_manager.update_object_by_entity(player, config.allies.color);
      }
    }
  }
}
