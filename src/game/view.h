#pragma once

#include <utils/pad.h>

#include <cstdint>

namespace game {
  struct ViewSetup {
    PAD(16);
    int32_t width;
    PAD(4);
    int32_t height;
    PAD(25);
    float fov;
  };
}
