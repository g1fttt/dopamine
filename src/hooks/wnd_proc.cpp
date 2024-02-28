#include "hooks.h"
#include "menu.h"

#include <app.h>
#include <menu.h>

#include <interfaces/input_system.h>

#include <imgui.h>

extern IMGUI_IMPL_API LRESULT ImGui_ImplWin32_WndProcHandler(HWND, UINT, WPARAM,
                                                             LPARAM);

LRESULT WINAPI hooks::wnd_proc(HWND window, UINT message, WPARAM wparam,
                               LPARAM lparam) {
  if (ImGui_ImplWin32_WndProcHandler(window, message, wparam, lparam)) {
    return true;
  }

  menu::handle_toggle();

  App::with([](App &app) {
    app.input_system->enable_input(!app.should_render_menu);
  });
  return CallWindowProcW(App::get().original_wnd_proc, window, message, wparam,
                         lparam);
}
