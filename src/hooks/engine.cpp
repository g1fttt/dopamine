#include "hooks.h"

#include <app.h>

float hooks::get_screen_aspect_ratio() {
  return App::with<float>([](const App &app) {
    const auto &aspect_ratio_cfg = app.config.misc.aspect_ratio;
    return aspect_ratio_cfg.enabled && !app.should_anti_screenshot()
               ? aspect_ratio_cfg.value
               : app.vmts.engine.call_original<float, 95>();
  });
}
