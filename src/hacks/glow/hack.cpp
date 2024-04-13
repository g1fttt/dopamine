#include "hack.h"

#include <game/entity.h>

#include <interfaces/engine.h>
#include <interfaces/entity_list.h>

#include <app.h>

#include "object_manager.h"

namespace glow {
  void Hack::manage_entities(ObjectManager &obj_manager, const App &app) const {
    for (int32_t i = 1; i < app.interfaces.engine->max_clients(); i += 1) {
      const auto entity = app.interfaces.entity_list->get_entity_by_index(i);
      if (!entity || !entity->is_player() || entity == app.local_player) {
        continue;
      }

      const auto player = entity->as<game::PlayerEntity>();
      const auto is_enemy = player->team() != app.local_player->team();

      obj_manager.register_entity(entity);

      if (config.enemies.enabled && is_enemy) {
        obj_manager.update_glow_color_for(player, config.enemies.color);
      } else if (config.allies.enabled && !is_enemy) {
        obj_manager.update_glow_color_for(player, config.allies.color);
      }
    }
  }
}
