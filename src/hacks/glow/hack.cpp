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
      if (!entity || !entity->is_player()) {
        continue;
      }

      const auto player = entity->as<game::PlayerEntity>();
      const auto is_enemy = player->team() != app.local_player->team();

      if (config.enemies.enabled && is_enemy) {
        obj_manager.register_object({player, config.enemies.color});
      } else if (config.allies.enabled && !is_enemy) {
        obj_manager.register_object({player, config.allies.color});
      } else {
        obj_manager.unregister_object_by_entity(entity);
      }
    }
  }
}
