#pragma once

#include <utils/vmethod.h>

namespace game
{
  struct MaterialVar {
    VMETHOD(void, set_value, 3, (float value), (this, value))
  };
}
