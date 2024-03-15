#pragma once

#include <utils/pad.h>

#include <cstdint>

namespace internal {
  class UserCommand {
  public:
    enum Command {
      InJump = 1 << 1,
    };
  public:
    PAD(36);
    int32_t buttons;
  };
}
