#include "hooks.h"

#include <hacks/visuals.h>

#include <app.h>

namespace engine {
  float STDCALL get_screen_aspect_ratio() {
    const auto &cfg = hacks::visuals.config.aspect_ratio;
    return cfg.enabled && !core::app->should_anti_screenshot()
               ? cfg.value
               : hooks->get_screen_aspect_ratio.call_original();
  }
}
