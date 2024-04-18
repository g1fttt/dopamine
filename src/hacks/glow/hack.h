#pragma once

#include <utils/color.h>

#include <config.h>

namespace glow {
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

    void manage_entities() const;

    Config config;
  };

  constinit inline Hack hack{};
}
