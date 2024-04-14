#pragma once

#include <utils/singleton.h>

#include <config.h>

namespace game {
  struct UserCommand;
}

namespace hacks {
  struct Misc : utils::Singleton<Misc> {
    struct Bunnyhop {
      // clang-format off
      DERIVE_SERDE(Bunnyhop,
        FIELD(enabled, "enabled")
        FIELD(chance, "chance"))
      // clang-format on

      bool enabled = false;
      float chance = 100.0f;
    };

    struct Config {
      // clang-format off
      DERIVE_SERDE(Config,
        FIELD(bunnyhop, "bunnyhop"))
      // clang-format on

      Bunnyhop bunnyhop;
    };

    void bunnyhop(game::UserCommand *cmd) const;

    Config config;
  };
}
