#include "menu.h"

#include <Windows.h>

#include "app.h"

#include "interfaces/input_system.h"

#include <imgui.h>

void Menu::render() const {
  if (open && toggle_animation_end < 1.0f) {
    ImGui::SetNextWindowFocus();
  }

  ImGui::PushStyleVar(ImGuiStyleVar_Alpha, get_transparency());
  {
    if (!open) {
      goto end;
    }
    ImGui::ShowDemoWindow();
  }
end:
  ImGui::PopStyleVar();
}

void Menu::update_animation() {
  toggle_animation_end += ImGui::GetIO().DeltaTime / animation_len();
}

void Menu::handle_toggle() {
  if (GetAsyncKeyState(VK_INSERT) & 1) {
    open = !open;
    if (!open) {
      App::get().input_system->reset_input_state();
    }

    if (toggle_animation_end > 0.0f && toggle_animation_end < 1.0f) {
      toggle_animation_end = 1.0f - toggle_animation_end;
    } else {
      toggle_animation_end = 0.0f;
    }

    ImGui::GetIO().MouseDrawCursor = open;
    ShowCursor(!open);
  }
}
