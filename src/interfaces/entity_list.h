#pragma once

#include <utils/vmethod.h>

#include <cstdint>

namespace internal {
  struct Entity;
}

namespace interfaces {
  struct EntityList {
    VMETHOD(internal::Entity *, get_entity_by_index, 3, (int32_t index),
            (this, index))
  };
}
