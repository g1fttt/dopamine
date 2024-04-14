#pragma once

#include <utils/vmethod.h>

#include <cstdint>

namespace game {
  struct Entity;

  struct EntityList {
    VMETHOD(Entity *, get_entity_by_index, 3, (int32_t index), (this, index))
    VMETHOD(Entity *, get_entity_from_handle, 4, (int32_t handle),
            (this, handle))
  };
}
