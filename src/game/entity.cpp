#include "entity.h"

#include <interfaces/entity_list.h>

#include <app.h>

namespace game {
  Entity *Entity::move_child() {
    return App::get().interfaces.entity_list->get_entity_from_handle(
        *utils::Ptr{this + 0x184}.cast<int32_t>());
  }

  Entity *Entity::move_peer() {
    return App::get().interfaces.entity_list->get_entity_from_handle(
        *utils::Ptr{this + 0x188}.cast<int32_t>());
  }
}
