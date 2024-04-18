#include "hooks.h"

#include <game/entity.h>

#include <game/engine.h>
#include <game/entity_list.h>

#include <hacks/glow/object_manager.h>

#include <app.h>

namespace hooks {
  void STDCALL level_init_post_entity() {
    app->hooks->level_init_post_entity.call_original();

    const auto entity = app->interfaces.entity_list->get_entity_by_index(
        app->interfaces.engine->local_player_index());
    app->local_player = entity->as<game::PlayerEntity>();
  }

  void STDCALL level_shutdown() {
    app->hooks->level_shutdown.call_original();
    app->local_player = nullptr;

    glow::object_manager->clear_objects();
  }
}
