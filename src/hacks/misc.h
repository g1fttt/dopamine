#pragma once

#include <config.h>

namespace game
{
  struct UserCommand;
  struct PlayerEntity;
}

namespace hacks
{
  struct Misc {
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

    void bunnyhop(game::PlayerEntity *local_player,
                  game::UserCommand *cmd) const;

    Config config;
  };

  constinit inline hacks::Misc misc{};
}
