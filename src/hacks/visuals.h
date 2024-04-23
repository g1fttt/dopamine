#pragma once

#include <utils/color.h>

#include <config.h>

struct ImDrawList;

namespace game {
  struct ViewSetup;
  struct PlayerEntity;
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

      core::config::Feature<float> aspect_ratio = {.value = 1.0f};
      core::config::Feature<float> add_fov = {.value = 10.0f};
      bool anti_screenshot = false;
      SniperRifleCrosshair sniper_rifle_crosshair;
    };

    void draw_sniper_crosshair(game::PlayerEntity *local_player,
                               ImDrawList *draw_list) const;
    void override_fov(game::ViewSetup *view) const;

    Config config;
  };

  constinit inline hacks::Visuals visuals{};
}
