#include <hacks/visuals.h>

#include <app.h>

namespace engine {
  float STDCALL get_screen_aspect_ratio() {
    const auto &cfg = hacks::visuals.config.aspect_ratio;
    return cfg.enabled && !app->should_anti_screenshot()
               ? cfg.value
               : app->hooks->get_screen_aspect_ratio.call_original();
  }
}
