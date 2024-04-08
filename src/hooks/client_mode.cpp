#include "hooks.h"

#include <game/view.h>

#include <interfaces/engine.h>
#include <interfaces/entity_list.h>

#include <hacks/misc.h>
#include <hacks/visuals.h>

#include <app.h>

bool STDCALL hooks::create_move(float input_sample_frame_time,
                                game::UserCommand *cmd) {
  const auto result = App::get().vmts.client_mode.call_original<bool, 21>(
      input_sample_frame_time, cmd);

  const auto &misc = hacks::Misc::get();
  { misc.bunnyhop(cmd); }

  return result;
}

void STDCALL hooks::override_view(game::ViewSetup *view) {
  App::get().vmts.client_mode.call_original<void, 16>(view);

  if (const auto &fov = hacks::Visuals::get().config.fov; fov.enabled) {
    view->fov = fov.value;
  }
}
