#pragma once

#include <utils/vmethod.h>

#include <cstdint>

namespace internal {
  class ConVar {
  public:
    VMETHOD(void, set_value, 12, (int32_t value), (this, value))
  };
}
