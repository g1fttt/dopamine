#pragma once

#include <app.h>
#include <utils/vmethod.h>

namespace game
{
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
    template <typename T> inline T *as() {
      return reinterpret_cast<T *>(this);
    }

    Entity *move_child() {
      return app->interfaces->entity_list->get_entity_from_handle(
          *utils::Ptr{this + 0x184}.cast<int32_t>());
    }

    Entity *move_peer() {
      return app->interfaces->entity_list->get_entity_from_handle(
          *utils::Ptr{this + 0x188}.cast<int32_t>());
    }

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
    bool is_sniper_rifle() {
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

    bool is_rifle_with_scope() {
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

    bool is_in_scope() {
      return is_rifle_with_scope() && mode() == WeaponMode::Secondary;
    }

    VMETHOD(WeaponID, id, 365, (), (this))

    NETVAR(WeaponMode, mode, "CWeaponCSBase", "m_weaponMode")
  };

  struct PlayerEntity : Entity {
    enum Flag {
      OnGround = 1 << 0,
    };

    static void init_methods(const core::Patterns &patterns) {
      METHOD_FROM_PATTERN_2(is_local_player);
    }

    inline bool is_local_player() {
      return methods.is_local_player(this);
    }

    inline bool is_on_ground() {
      return flags() & OnGround;
    }

    VMETHOD(WeaponEntity *, active_weapon, 222, (), (this))

    NETVAR(int32_t, team, "CBaseEntity", "m_iTeamNum")
    NETVAR(Flag, flags, "CBasePlayer", "m_fFlags")
  private:
    struct Methods {
      bool(THISCALL *is_local_player)(Entity *);
    };

    inline static Methods methods{};
  };
}
