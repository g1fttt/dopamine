#include "hooks.h"

#include <game/entity.h>

#include <game/engine.h>
#include <game/entity_list.h>

#include <hacks/glow/object_manager.h>

#include <app.h>
#include <interfaces.h>

namespace client {
  void STDCALL level_init_post_entity() {
    hooks->level_init_post_entity.call_original();

    const auto entity = core::interfaces->entity_list->get_entity_by_index(
        core::interfaces->engine->local_player_index());
    core::app->local_player = entity->as<game::PlayerEntity>();
  }

  void STDCALL level_shutdown() {
    hooks->level_shutdown.call_original();
    core::app->local_player = nullptr;

    glow::object_manager->clear_objects();
  }
}
