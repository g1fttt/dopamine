#pragma once

#include <utils/color.h>
#include <utils/singleton.h>

#include <config.h>

struct App;

namespace glow {
  struct ObjectManager;

  struct Hack : utils::Singleton<Hack> {
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

    void manage_entities(ObjectManager &obj_manager, const App &app) const;

    Config config;
  };
}
