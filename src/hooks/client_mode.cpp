#include <game/engine.h>
#include <game/entity.h>
#include <game/entity_list.h>
#include <game/view.h>

#include <hacks/glow/hack.h>

#include <hacks/misc.h>
#include <hacks/visuals.h>

#include <app.h>

namespace client_mode
{
  bool STDCALL create_move(float input_sample_frame_time,
                           game::UserCommand *cmd) {
    const auto result =
        app->hooks->create_move.call_original(input_sample_frame_time, cmd);

    hacks::misc.bunnyhop(app->local_player, cmd);

    return result;
  }

  void STDCALL override_view(game::ViewSetup *view) {
    app->hooks->override_view.call_original(view);

    hacks::visuals.override_fov(view);
  }

  bool STDCALL do_post_screen_space_effects(const game::ViewSetup *view) {
    const auto result =
        app->hooks->do_post_screen_space_effects.call_original(view);

    const auto &interfaces = *app->interfaces;

    if (interfaces.engine->is_in_game()) {
      auto &glow_object_manager = *app->glow_object_manager;

      glow::hack.manage_entities(interfaces, glow_object_manager,
                                 app->local_player);
      glow_object_manager.draw_glow_effects(interfaces, view);
      glow_object_manager.force_disable();
    }
    return result;
  }
}
