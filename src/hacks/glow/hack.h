#pragma once

#include <utils/color.h>

#include <config.h>

namespace game {
  struct PlayerEntity;
}

namespace core {
  struct Interfaces;
}

namespace glow {
  struct ObjectManager;

  struct Hack {
    struct Glow {
      // clang-format off
      DERIVE_SERDE(Glow,
        FIELD(enabled, "enabled")
        FIELD(color, "color"))
      // clang-format on

      bool enabled = false;
      utils::Color color;
    };

    struct Config {
      // clang-format off
      DERIVE_SERDE(Config,
        FIELD(enemies, "enemies")
        FIELD(allies, "allies"))
      // clang-format on

      Glow enemies;
      Glow allies;
    };

    void manage_entities(const core::Interfaces &interfaces,
                         ObjectManager &object_manager,
                         game::PlayerEntity *local_player) const;

    Config config;
  };

  constinit inline Hack hack{};
}
