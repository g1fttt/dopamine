#include "app.h"

#include <d3d9.h>

#include "hacks/visuals.h"

#include "game/engine.h"
#include "game/input_system.h"

#include "config.h"

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
  void App::setup() {
    config::init_or_nothing();

    glow_object_manager = glow::ObjectManager{*interfaces, material_creator};

    hooks->setup(*interfaces, *patterns, window);
  }

  bool App::should_anti_screenshot() const {
    return hacks::visuals.config.anti_screenshot &&
           interfaces->engine->is_taking_screenshot();
  }

  void App::unload() {
    ShowCursor(TRUE);

    hooks->remove(*patterns);

    ImGui_ImplWin32_Shutdown();
    ImGui_ImplDX9_Shutdown();

    const auto handle = CreateThread(
        nullptr, 0, LPTHREAD_START_ROUTINE(reset_state), this, 0, nullptr);
    if (handle) {
      CloseHandle(handle);
    }
  }
}
