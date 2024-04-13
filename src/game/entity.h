#pragma once

#include <utils/netvars.h>
#include <utils/pad.h>
#include <utils/vmethod.h>

namespace game {
  struct NetworkableEntity {
    VMETHOD(bool, is_dormant, 8, (), (this))
  };

  enum class DrawModelFlag {
    StudioRender = 1,
  };

  struct RenderableEntity {
    VMETHOD(bool, should_draw, 3, (), (this))
    VMETHOD(int32_t, draw_model, 10,
            (DrawModelFlag flags = DrawModelFlag::StudioRender), (this, flags))
  };

  struct Entity {
    template <typename T> constexpr T *as() {
      return reinterpret_cast<T *>(this);
    }

    Entity *move_child();
    Entity *move_peer();

    VMETHOD(NetworkableEntity *, networkable, 4, (), (this))
    VMETHOD(RenderableEntity *, renderable, 5, (), (this))
    VMETHOD(bool, is_player, 131, (), (this))
  };

  enum WeaponID {
    Scout = 3,
    SG550 = 13,
    AWP = 17,
    G3SG1 = 23,
  };

  struct WeaponEntity : Entity {
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

  struct PlayerEntity : Entity {
    enum Flag {
      OnGround = 1 << 0,
    };

    constexpr bool is_on_ground() {
      return flags() & OnGround;
    }

    VMETHOD(WeaponEntity *, active_weapon, 222, (), (this))

    NETVAR(int32_t, team, "CBaseEntity", "m_iTeamNum")
    NETVAR(Flag, flags, "CBasePlayer", "m_fFlags")
  };
}
