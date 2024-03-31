#include "hooks.h"

#include <internal/view.h>

#include <interfaces/engine.h>
#include <interfaces/entity_list.h>

#include <hacks/misc.h>
#include <hacks/visuals.h>

#include <app.h>

// Everyone checking cmd pointer for nullness but, i think there is no
// need for this because as i saw, there are no functions that push nullptr to
// this particular function
bool STDCALL hooks::create_move(float input_sample_frame_time,
                                internal::UserCommand *cmd) {
  return App::with<bool>([&](App &app) {
    // Local player pointer is always non-null (when we in-game, so that is the
    // reason why we get it inside create_move hook)
    app.local_player = app.interfaces.entity_list->get_entity_by_index(
        app.interfaces.engine->get_local_player_index());

    const auto result = app.vmts.client_mode.call_original<bool, 21>(
        input_sample_frame_time, cmd);

    const auto &misc = hacks::Misc::get();
    { misc.bunnyhop(cmd); }

    return result;
  });
}

void STDCALL hooks::override_view(internal::ViewSetup *view) {
  App::with<void>([&](const App &app) {
    app.vmts.client_mode.call_original<void, 16>(view);

    const auto &visuals = hacks::Visuals::get();
    {
      if (const auto &fov = visuals.config.fov; fov.enabled) {
        view->fov = fov.value;
      }
    }
  });
}
