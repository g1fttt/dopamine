#pragma once

#include <utils/vmethod.h>

#include <cstdint>

namespace internal {
  class Entity;
}

namespace interfaces {
  class EntityList {
  public:
    VMETHOD(internal::Entity *, get_entity_by_index, 3, (int32_t index),
            (this, index))
  };
}
