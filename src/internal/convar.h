#pragma once

#include <utils/vmethod.h>

#include <cstdint>

namespace internal {
  struct ConVar {
    VMETHOD(void, set_value, 12, (int32_t value), (this, value))
  };
}
