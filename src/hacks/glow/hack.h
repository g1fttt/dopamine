#pragma once

#include <utils/color.h>

#include <config.h>

struct App;

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

    constexpr Hack(const Hack &) = delete;

    static Hack &get() {
      static Hack self{};
      return self;
    }

    void manage_entities(ObjectManager &obj_manager, const App &app) const;

    Config config;
  private:
    constexpr Hack() = default;
  };
}
