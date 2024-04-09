#include "hooks.h"

#include <interfaces/engine.h>
#include <interfaces/entity_list.h>

#include <app.h>

namespace hooks {
  void STDCALL level_init_post_entity() {
    App::get().and_then<void>([](App &app) {
      app.hooks->level_init_post_entity.call_original();
      app.local_player = app.interfaces.entity_list->get_entity_by_index(
          app.interfaces.engine->local_player_index());
    });
  }

  void STDCALL level_shutdown() {
    App::get().and_then<void>([](App &app) {
      app.hooks->level_shutdown.call_original();
      app.local_player = nullptr;
    });
  }
}
