#pragma once

#include <utils/color.h>

#include <config.h>

struct ImDrawList;

namespace {
  using config::Feature;
  using utils::Color;
}

namespace hacks {
  struct Visuals {
    struct SniperRifleCrosshair {
      // clang-format off
      DERIVE_SERDE(SniperRifleCrosshair,
        FIELD(enabled, "enabled")
        FIELD(size, "size")
        FIELD(thickness, "thickness")
        FIELD(color, "color"))
      // clang-format on

      bool enabled = false;
      float size = 10.0f;
      float thickness = 1.0f;
      Color color = Color::WHITE;
    };

    struct Config {
      // clang-format off
      DERIVE_SERDE(Config,
        FIELD(aspect_ratio, "aspect-ratio")
        FIELD(fov, "fov")
        FIELD(anti_screenshot, "anti-screenshot")
        FIELD(sniper_rifle_crosshair, "sniper-rifle-crosshair"))
      // clang-format on

      Feature<float> aspect_ratio = {.value = 1.0f};
      Feature<float> fov = {.value = 70.0f};
      bool anti_screenshot = false;
      SniperRifleCrosshair sniper_rifle_crosshair;
    };

    constexpr Visuals(const Visuals &) = delete;

    static Visuals &get() {
      static Visuals self{};
      return self;
    }

    void draw_sniper_crosshair(ImDrawList *draw_list) const;

    Config config;
  private:
    constexpr Visuals() = default;
  };
}
