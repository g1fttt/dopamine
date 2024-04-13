#include "hooks.h"

#include <game/entity.h>

#include <interfaces/engine.h>
#include <interfaces/entity_list.h>

#include <hacks/glow/object_manager.h>

#include <app.h>

namespace hooks {
  void STDCALL level_init_post_entity() {
    App::get().and_then<void>([](App &app) {
      app.hooks->level_init_post_entity.call_original();

      const auto entity = app.interfaces.entity_list->get_entity_by_index(
          app.interfaces.engine->local_player_index());
      app.local_player = entity->as<game::PlayerEntity>();
    });
  }

  void STDCALL level_shutdown() {
    App::get().and_then<void>([](App &app) {
      app.hooks->level_shutdown.call_original();
      app.local_player = nullptr;

      glow::ObjectManager::get().clear_objects();
    });
  }
}
