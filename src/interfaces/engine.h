#pragma once

#include <utils/vmethod.h>

#include <cstdint>

namespace interfaces {
  class Engine {
  public:
    VMETHOD(int32_t, get_local_player_index, 12, (), (this))
  };
}
