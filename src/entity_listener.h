#pragma once

#include "game/entity_list.h"

#include <forward_list>

namespace game
{
  struct PlayerEntity;
}

namespace core
{
  struct EntityListener : game::EntityListener {
    void on_entity_created(game::Entity *entity) override;
    void on_entity_deleted(game::Entity *entity) override;

    std::forward_list<game::PlayerEntity *> players;
  };
}
