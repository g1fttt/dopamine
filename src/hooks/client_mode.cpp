#include "hooks.h"

#include <game/view.h>

#include <interfaces/engine.h>
#include <interfaces/entity_list.h>

#include <hacks/misc.h>
#include <hacks/visuals.h>

#include <app.h>

bool STDCALL hooks::create_move(float input_sample_frame_time,
                                game::UserCommand *cmd) {
  const auto result =
      App::get().hooks.create_move.call_original(input_sample_frame_time, cmd);

  const auto &misc = hacks::Misc::get();
  { misc.bunnyhop(cmd); }

  return result;
}

void STDCALL hooks::override_view(game::ViewSetup *view) {
  App::get().and_then<void>([=](const App &app) {
    app.hooks.override_view.call_original(view);

    if (const auto &fov = hacks::Visuals::get().config.fov;
        fov.enabled && !app.should_anti_screenshot()) {
      view->fov = fov.value;
    }
  });
}
