#include <game/input_system.h>
#include <ui/menu.h>

#include <app.h>
#include <input.h>

#include <imgui.h>

extern IMGUI_IMPL_API LRESULT ImGui_ImplWin32_WndProcHandler(HWND, UINT, WPARAM,
                                                             LPARAM);

namespace winapi {
  LRESULT WINAPI wnd_proc(HWND window, UINT message, WPARAM wparam,
                          LPARAM lparam) {
    if (ImGui_ImplWin32_WndProcHandler(window, message, wparam, lparam)) {
      return true;
    }
    core::input.with(message, wparam, lparam, [&](const core::Input &input) {
      ui::menu.handle_toggle(input);

      if (input.key_is_up(VK_END)) {
        app->should_unload = true;
      }
    });
    return CallWindowProcW(app->hooks->wnd_proc_original, window, message,
                           wparam, lparam);
  }
}
