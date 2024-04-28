#include <d3d9.h>

#include "hacks/visuals.h"

#include "game/engine.h"
#include "game/entity.h"
#include "game/input_system.h"
#include "game/key_values.h"

#include "config.h"

#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

namespace core
{
  App::App(HMODULE module)
      : module{module}
      , window{FindWindowA("Valve001", nullptr)}
      , hooks{std::make_optional<Hooks>()}
      , interfaces{Interfaces{}}
      , patterns{Patterns{}}
      , netvars{*interfaces} {
    DisableThreadLibraryCalls(module);

    config::init_or_nothing();

    std::atexit([] {
      config::save();
    });

    game::PlayerEntity::init_methods(*patterns);
    game::EntityList::init_methods(*patterns);
    game::KeyValues::init_methods(*patterns);

    glow_object_manager = glow::ObjectManager{*interfaces, material_creator};

    interfaces->entity_list->add_entity_listener(&entity_listener);

    hooks->setup(*interfaces, *patterns, window);
  }

  App::~App() {
    interfaces->input_system->enable_input(true);

    hooks->remove(*patterns);

    // I didn't found CGlobalEntityList::RemoveEntityListener, so our entity
    // listener won't be removed from CUtlVector until game close :(
    //
    // Fortunately, it won't cause any (i presume?) crashes

    ImGui_ImplWin32_Shutdown();
    ImGui_ImplDX9_Shutdown();

    FreeLibraryAndExitThread(module, EXIT_SUCCESS);
  }

  bool App::should_anti_screenshot() const {
    return hacks::visuals.config.anti_screenshot &&
           interfaces->engine->is_taking_screenshot();
  }
}
