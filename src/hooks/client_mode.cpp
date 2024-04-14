#include "hooks.h"

#include <game/entity.h>
#include <game/view.h>

#include <interfaces/engine.h>
#include <interfaces/entity_list.h>

#include <hacks/glow/hack.h>
#include <hacks/glow/object_manager.h>

#include <hacks/misc.h>
#include <hacks/visuals.h>

#include <app.h>

namespace hooks {
  bool STDCALL create_move(float input_sample_frame_time,
                           game::UserCommand *cmd) {
    const auto result = App::get().hooks->create_move.call_original(
        input_sample_frame_time, cmd);

    const auto &misc = hacks::Misc::get();
    { misc.bunnyhop(cmd); }

    return result;
  }

  void STDCALL override_view(game::ViewSetup *view) {
    App::get().and_then<void>([=](const App &app) {
      app.hooks->override_view.call_original(view);

      if (const auto &fov = hacks::Visuals::get().config.fov;
          fov.enabled && !app.should_anti_screenshot()) {
        view->fov = fov.value;
      }
    });
  }

  bool STDCALL do_post_screen_space_effects(const game::ViewSetup *view) {
    return App::get().and_then<bool>([=](App &app) {
      if (app.interfaces.engine->is_in_game()) {
        auto &glow_object_manager =
            glow::ObjectManager::get_or_init(glow::ObjectManager::init_func(
                app.interfaces.material_system.get()));
        { glow::Hack::get().manage_entities(glow_object_manager, app); }
        glow_object_manager.draw_glow_effects(view, app);
        glow_object_manager.force_disable();
      }
      return app.hooks->do_post_screen_space_effects.call_original(view);
    });
  }
}
