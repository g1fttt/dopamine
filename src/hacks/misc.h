#pragma once

#include <config.h>

namespace internal {
  class UserCommand;
}

namespace hacks {
  struct Misc {
    constexpr Misc(const Misc &&) = delete;
    constexpr Misc(const Misc &) = delete;

    static Misc &get() {
      static Misc self{};
      return self;
    }

    void bunnyhop(internal::UserCommand *cmd) const;

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

    Config config;
  private:
    constexpr Misc() = default;
  };
}
