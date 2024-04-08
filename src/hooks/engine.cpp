#include "hooks.h"

#include <hacks/visuals.h>

#include <app.h>

float hooks::get_screen_aspect_ratio() {
  return App::get().and_then<float>([](const App &app) {
    const auto &aspect_ratio = hacks::Visuals::get().config.aspect_ratio;
    return aspect_ratio.enabled && !app.should_anti_screenshot()
               ? aspect_ratio.value
               : app.hooks.get_screen_aspect_ratio.call_original();
  });
}
