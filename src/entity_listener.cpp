#include "entity_listener.h"

#include <game/entity.h>
#include <hacks/glow/hack.h>

namespace core
{
  void EntityListener::on_entity_created(game::Entity *entity) {
    if (!entity->is_player()) {
      return;
    }

    if (const auto player = entity->as<game::PlayerEntity>();
        !player->is_local_player())
    {
      players.push_front(player);

      if (auto &glow_object_manager = *app->glow_object_manager;
          !glow_object_manager.has_glow_effect(entity))
      {
        glow_object_manager.register_entity(entity);
      }
    }
  }

  void EntityListener::on_entity_deleted(game::Entity *entity) {
    if (!entity->is_player())
      return;

    if (const auto player = entity->as<game::PlayerEntity>();
        !player->is_local_player())
    {
      app->glow_object_manager->unregister_object_by_entity(entity);

      players.remove(player);
    }
  }
}
