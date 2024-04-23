#include "entity.h"

#include <game/entity_list.h>

namespace game {
  Entity *Entity::move_child() {
    return app->interfaces->entity_list->get_entity_from_handle(
        *utils::Ptr{this + 0x184}.cast<int32_t>());
  }

  Entity *Entity::move_peer() {
    return app->interfaces->entity_list->get_entity_from_handle(
        *utils::Ptr{this + 0x188}.cast<int32_t>());
  }
}
