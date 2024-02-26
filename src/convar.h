#pragma once

#include "vmethod.h"

#include <cstdint>

class ConVar {
public:
  VMETHOD(void, set_value, 12, (int32_t value), (this, value))
};
