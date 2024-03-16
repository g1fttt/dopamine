#pragma once

#include <utils/fnv_hash.h>
#include <utils/netvars.h>

namespace internal {
  class Entity {
  public:
    enum Flag {
      OnGround = 1 << 0,
    };
  public:
    constexpr bool is_on_ground() {
      return flags() & OnGround;
    }

    NETVAR(Flag, flags, "CBasePlayer", "m_fFlags")
  };
}
