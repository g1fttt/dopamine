#include "hooks.h"

#include <interfaces/engine.h>
#include <interfaces/entity_list.h>

#include <app.h>

namespace hooks {
  void STDCALL frame_stage_notify(int32_t stage) {
    App::get().and_then<void>([=](App &app) {
      app.local_player = app.interfaces.entity_list->get_entity_by_index(
          app.interfaces.engine->local_player_index());
      return app.hooks->frame_stage_notify.call_original(stage);
    });
  }
}
