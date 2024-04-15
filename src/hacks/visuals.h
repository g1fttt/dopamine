#pragma once

#include <utils/color.h>
#include <utils/singleton.h>

#include <config.h>

struct ImDrawList;

namespace game {
  struct ViewSetup;
}

struct App;

namespace hacks {
  struct Visuals : utils::Singleton<Visuals> {
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
      utils::Color color;
    };

    struct Config {
      // clang-format off
      DERIVE_SERDE(Config,
        FIELD(aspect_ratio, "aspect-ratio")
        FIELD(add_fov, "add-fov")
        FIELD(anti_screenshot, "anti-screenshot")
        FIELD(sniper_rifle_crosshair, "sniper-rifle-crosshair"))
      // clang-format on

      config::Feature<float> aspect_ratio = {.value = 1.0f};
      config::Feature<float> add_fov = {.value = 10.0f};
      bool anti_screenshot = false;
      SniperRifleCrosshair sniper_rifle_crosshair;
    };

    void draw_sniper_crosshair(ImDrawList *draw_list, const App &app) const;
    void override_fov(game::ViewSetup *view, const App &app) const;

    Config config;
  };
}
