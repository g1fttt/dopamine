#pragma once

#include <utils/pad.h>

#include <cstdint>

namespace internal {
  class Entity {
  public:
    constexpr bool is_on_ground() const {
      return flags & Flag::OnGround;
    }
  private:
    enum Flag {
      OnGround = 1 << 0,
    };
  private:
    PAD(0x350);
    int32_t flags;
  };
}
