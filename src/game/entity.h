#pragma once

#include <utils/vmethod.h>

#include <app.h>

namespace game {
  struct NetworkableEntity {
    VMETHOD(bool, is_dormant, 8, (), (this))
  };

  enum struct DrawModelFlag {
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

  enum struct WeaponID {
    Scout = 3,
    AUG = 8,
    SG550 = 13,
    AWP = 17,
    G3SG1 = 23,
    SG552 = 26,
  };

  enum struct WeaponMode {
    Secondary = 1,
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

    constexpr bool is_rifle_with_scope() {
      if (!is_sniper_rifle()) {
        switch (id()) {
        case WeaponID::AUG:
        case WeaponID::SG552:
          return true;
        default:
          return false;
        }
      } else {
        return true;
      }
    }

    constexpr bool is_in_scope() {
      return is_rifle_with_scope() && mode() == WeaponMode::Secondary;
    }

    VMETHOD(WeaponID, id, 365, (), (this))

    NETVAR(WeaponMode, mode, "CWeaponCSBase", "m_weaponMode")
  };

  struct PlayerEntity : Entity {
    enum Flag {
      OnGround = 1 << 0,
    };

    constexpr bool is_on_ground() const {
      return flags() & OnGround;
    }

    VMETHOD(WeaponEntity *, active_weapon, 222, (), (this))

    NETVAR(int32_t, team, "CBaseEntity", "m_iTeamNum")
    NETVAR(Flag, flags, "CBasePlayer", "m_fFlags")
  };
}
