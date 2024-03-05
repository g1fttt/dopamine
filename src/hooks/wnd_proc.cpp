#include "hooks.h"

#include <app.h>

#include <interfaces/input_system.h>
#include <ui/menu.h>
#include <utils/input.h>

#include <imgui.h>

extern IMGUI_IMPL_API LRESULT ImGui_ImplWin32_WndProcHandler(HWND, UINT, WPARAM,
                                                             LPARAM);

LRESULT WINAPI hooks::wnd_proc(HWND window, UINT message, WPARAM wparam,
                               LPARAM lparam) {
  if (ImGui_ImplWin32_WndProcHandler(window, message, wparam, lparam)) {
    return true;
  }

  utils::Input::with(message, wparam, lparam, [](const utils::Input &input) {
    ui::Menu::get().handle_toggle();

    if (input.key_is_up(VK_END)) {
      App::get().should_unhook = true;
    }
  });
  return CallWindowProcW(App::get().original_wnd_proc, window, message, wparam,
                         lparam);
}
