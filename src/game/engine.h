#pragma once

#include <utils/vmethod.h>

#include <cstdint>

namespace game
{
  struct Engine {
    VMETHOD(int32_t, local_player_index, 12, (), (this))
    VMETHOD(bool, is_in_game, 26, (), (this))
    VMETHOD(bool, is_taking_screenshot, 85, (), (this))
  };
}
