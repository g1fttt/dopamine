#include <d3d9.h>

#include "hacks/visuals.h"

#include "game/engine.h"
#include "game/entity.h"
#include "game/input_system.h"
#include "game/key_values.h"

#include "config.h"
#include "entity_listener.h"

#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

static void STDCALL reset_state(core::App *app) {
  app->interfaces->input_system->enable_input(true);

  // This will trigger DllMain with DLL_PROCESS_DETACH and app pointer will be
  // destroyed properly
  FreeLibraryAndExitThread(app->module, EXIT_SUCCESS);
}

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

    // Allow entity_listener to stay in static memory, so game won't crash
    // trying to access invalid memory address after hack unloading
    static EntityListener entity_listener{};
    interfaces->entity_list->add_entity_listener(&entity_listener);

    hooks->setup(*interfaces, *patterns, window);
  }

  App::~App() {
    hooks->remove(*patterns);

    // I didn't found CGlobalEntityList::RemoveEntityListener, so our entity
    // listener won't be removed from CUtlVector until game close :(

    ImGui_ImplWin32_Shutdown();
    ImGui_ImplDX9_Shutdown();
  }

  bool App::should_anti_screenshot() const {
    return hacks::visuals.config.anti_screenshot &&
           interfaces->engine->is_taking_screenshot();
  }

  void App::unload() {
    ShowCursor(TRUE);

    const auto handle = CreateThread(
        nullptr, 0, LPTHREAD_START_ROUTINE(reset_state), this, 0, nullptr);
    if (handle) {
      CloseHandle(handle);
    }
  }
}
