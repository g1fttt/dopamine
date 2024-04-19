#include "app.h"

#include <d3d9.h>

#include "hacks/glow/object_manager.h"
#include "ui/post_processing.h"

#include "hacks/visuals.h"
#include "hooks/hooks.h"

#include "utils/netvars.h"
#include "utils/patterns.h"

#include "game/engine.h"
#include "game/input_system.h"

#include "config.h"
#include "interfaces.h"

#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

namespace core {
  App::App(HMODULE module) {
    hooks = std::make_unique<Hooks>();

    this->module = module;
    DisableThreadLibraryCalls(module);

    config::init_or_nothing();

    std::atexit([] {
      config::save();
    });

    window = FindWindowA("Valve001", nullptr);

    interfaces = Interfaces{};

    utils::patterns = utils::Patterns{};

    glow::object_manager = glow::ObjectManager{};
    ui::blur_effect = ui::BlurEffect{};

    hooks->setup(window);

    utils::netvars = utils::Netvars{};
  }

  void App::reset() {
    interfaces->input_system->enable_input(true);

    hooks->remove();

    ImGui_ImplWin32_Shutdown();
    ImGui_ImplDX9_Shutdown();

    FreeLibraryAndExitThread(module, EXIT_SUCCESS);
  }

  bool App::should_anti_screenshot() const {
    return hacks::visuals.config.anti_screenshot &&
           interfaces->engine->is_taking_screenshot();
  }
}
