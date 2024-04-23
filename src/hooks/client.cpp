#include <game/entity.h>

#include <game/engine.h>
#include <game/entity_list.h>

#include <app.h>

namespace client
{
  void STDCALL level_init_post_entity() {
    app->hooks->level_init_post_entity.call_original();

    const auto &interfaces = *app->interfaces;

    const auto entity = interfaces.entity_list->get_entity_by_index(
        interfaces.engine->local_player_index());
    app->local_player = entity->as<game::PlayerEntity>();
  }

  void STDCALL level_shutdown() {
    app->hooks->level_shutdown.call_original();
    app->local_player = nullptr;
    app->glow_object_manager->clear_objects();
  }
}
