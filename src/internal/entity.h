#pragma once

#include <utils/fnv_hash.h>
#include <utils/netvars.h>

namespace internal {
  struct Entity {
    enum Flag {
      OnGround = 1 << 0,
    };

    constexpr bool is_on_ground() {
      return flags() & OnGround;
    }

    NETVAR(Flag, flags, "CBasePlayer", "m_fFlags")
  };
}
