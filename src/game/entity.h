#pragma once

#include <utils/netvars.h>
#include <utils/vmethod.h>

namespace game {
  enum WeaponID {
    Scout = 3,
    SG550 = 13,
    AWP = 17,
    G3SG1 = 23,
  };

  struct WeaponEntity {
    constexpr bool is_sniper_rifle() {
      switch (id()) {
      case WeaponID::Scout:
      case WeaponID::SG550:
      case WeaponID::AWP:
      case WeaponID::G3SG1:
        return true;
      default:
        return false;
      }
    }

    VMETHOD(WeaponID, id, 365, (), (this))
  };

  struct PlayerEntity {
    enum Flag {
      OnGround = 1 << 0,
    };

    constexpr bool is_on_ground() {
      return flags() & OnGround;
    }

    VMETHOD(WeaponEntity *, active_weapon, 222, (), (this))

    NETVAR(Flag, flags, "CBasePlayer", "m_fFlags")
  };
}
