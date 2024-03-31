#pragma once

#include <config.h>

namespace hacks {
  struct Visuals {
    constexpr Visuals(const Visuals &&) = delete;
    constexpr Visuals(const Visuals &) = delete;

    static Visuals &get() {
      static Visuals self{};
      return self;
    }

    struct Config {
      // clang-format off
      DERIVE_SERDE(Config,
        FIELD(aspect_ratio, "aspect-ratio")
        FIELD(fov, "fov")
        FIELD(anti_screenshot, "anti-screenshot"))
      // clang-format on

      config::Feature<float> aspect_ratio = {.value = 1.0f};
      config::Feature<float> fov = {.value = 78.0f};
      bool anti_screenshot = false;
    };

    Config config;
  private:
    constexpr Visuals() = default;
  };
}
