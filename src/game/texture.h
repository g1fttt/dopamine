#pragma once

#include <utils/vmethod.h>

#include <cstdint>

namespace game
{
  struct Texture {
    VMETHOD(int32_t, actual_width, 3, (), (this))
    VMETHOD(int32_t, actual_height, 4, (), (this))
    VMETHOD(void, inc_ref_counter, 10, (), (this))
  };
}
