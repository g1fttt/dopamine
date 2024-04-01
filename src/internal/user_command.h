#pragma once

#include <utils/pad.h>

#include <cstdint>

namespace internal {
  struct UserCommand {
    enum Command {
      InJump = 1 << 1,
    };

    PAD(36);
    int32_t buttons;
  };
}
