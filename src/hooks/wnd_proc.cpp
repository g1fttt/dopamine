#include "hooks.h"

#include <app.h>
#include <menu.h>

#include <interfaces/input_system.h>
#include <utils/input.h>

#include <imgui.h>

extern IMGUI_IMPL_API LRESULT ImGui_ImplWin32_WndProcHandler(HWND, UINT, WPARAM,
                                                             LPARAM);

LRESULT WINAPI hooks::wnd_proc(HWND window, UINT message, WPARAM wparam,
                               LPARAM lparam) {
  if (ImGui_ImplWin32_WndProcHandler(window, message, wparam, lparam)) {
    return true;
  }

  utils::Input::with(message, wparam, lparam, [] {
    core::Menu::get().handle_toggle();
  });
  return CallWindowProcW(App::get().original_wnd_proc, window, message, wparam,
                         lparam);
}
