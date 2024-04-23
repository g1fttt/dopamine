#include "app.h"

#include <d3d9.h>

#include "hacks/visuals.h"

#include "game/engine.h"
#include "game/input_system.h"
#include "game/key_values.h"

#include "config.h"
#include "interfaces.h"

#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

namespace core {
  App::App(HMODULE module) : hooks{std::make_optional<Hooks>()} {
    this->module = module;
    DisableThreadLibraryCalls(module);

    config::init_or_nothing();

    std::atexit([] {
      config::save();
    });

    window = FindWindowA("Valve001", nullptr);

    interfaces = Interfaces{};
    patterns = Patterns{};
    netvars = Netvars{*interfaces};

    game::KeyValues::init_methods(*patterns);

    glow_object_manager = glow::ObjectManager{*interfaces};

    hooks->setup(*interfaces, *patterns, window);
  }

  void App::reset() {
    interfaces->input_system->enable_input(true);

    hooks->remove(*patterns);

    ImGui_ImplWin32_Shutdown();
    ImGui_ImplDX9_Shutdown();

    FreeLibraryAndExitThread(module, EXIT_SUCCESS);
  }

  bool App::should_anti_screenshot() const {
    return hacks::visuals.config.anti_screenshot &&
           interfaces->engine->is_taking_screenshot();
  }
}
