#pragma once

#include <utils/vmethod.h>

#include <cstdint>

namespace game {
  struct PlayerEntity;
}

namespace interfaces {
  struct EntityList {
    VMETHOD(game::PlayerEntity *, get_entity_by_index, 3, (int32_t index),
            (this, index))
  };
}
