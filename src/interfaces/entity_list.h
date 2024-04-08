#pragma once

#include <utils/vmethod.h>

#include <cstdint>

namespace internal {
  struct PlayerEntity;
}

namespace interfaces {
  struct EntityList {
    VMETHOD(internal::PlayerEntity *, get_entity_by_index, 3, (int32_t index),
            (this, index))
  };
}
