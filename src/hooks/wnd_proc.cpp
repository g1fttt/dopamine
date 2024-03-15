#include "hooks.h"

#include <app.h>

#include <interfaces/input_system.h>
#include <ui/menu.h>
#include <utils/input.h>

#include <imgui.h>

using utils::Input;

extern IMGUI_IMPL_API LRESULT ImGui_ImplWin32_WndProcHandler(HWND, UINT, WPARAM,
                                                             LPARAM);

LRESULT WINAPI hooks::wnd_proc(HWND window, UINT message, WPARAM wparam,
                               LPARAM lparam) {
  if (ImGui_ImplWin32_WndProcHandler(window, message, wparam, lparam)) {
    return true;
  }

  auto &app = App::get();

  Input::with(message, wparam, lparam, [&](const Input &input) {
    ui::Menu::get().handle_toggle();

    if (input.key_is_up(VK_END)) {
      app.should_unhook = true;
    }
  });
  return CallWindowProcW(app.original_wnd_proc, window, message, wparam,
                         lparam);
}
