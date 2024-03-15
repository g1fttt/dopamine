#include "hooks.h"

#include <internal/entity.h>
#include <internal/user_command.h>

#include <interfaces/engine.h>
#include <interfaces/entity_list.h>

#include <hacks/hacks.h>

#include <app.h>

// Everyone checking cmd pointer for nullness but, i think there is no
// need for this because as i saw, there are no functions that push nullptr to
// this particular function
bool STDCALL hooks::create_move(float input_sample_frame_time,
                                internal::UserCommand *cmd) {
  auto &app = App::get();

  const auto result = app.vmts.client_mode.call_original<bool, 21>(
      input_sample_frame_time, cmd);

  // Local player pointer is always non-null (when we in-game, so that is the
  // reason why we get it inside create_move hook)
  app.local_player = app.interfaces.entity_list->get_entity_by_index(
      app.interfaces.engine->get_local_player_index());

  hacks::bunnyhop(cmd);

  return result;
}
