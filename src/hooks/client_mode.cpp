#include "hooks.h"

#include <game/engine.h>
#include <game/entity.h>
#include <game/entity_list.h>
#include <game/view.h>

#include <hacks/glow/hack.h>
#include <hacks/glow/object_manager.h>

#include <hacks/misc.h>
#include <hacks/visuals.h>

#include <interfaces.h>

namespace client_mode {
  bool STDCALL create_move(float input_sample_frame_time,
                           game::UserCommand *cmd) {
    const auto result =
        hooks->create_move.call_original(input_sample_frame_time, cmd);

    hacks::misc.bunnyhop(cmd);

    return result;
  }

  void STDCALL override_view(game::ViewSetup *view) {
    hooks->override_view.call_original(view);

    hacks::visuals.override_fov(view);
  }

  bool STDCALL do_post_screen_space_effects(const game::ViewSetup *view) {
    const auto result = hooks->do_post_screen_space_effects.call_original(view);

    if (core::interfaces->engine->is_in_game()) {
      glow::hack.manage_entities();
      glow::object_manager->draw_glow_effects(view);
      glow::object_manager->force_disable();
    }
    return result;
  }
}
