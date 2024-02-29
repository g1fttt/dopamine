#include "menu.h"

#include <Windows.h>

#include "app.h"

#include "interfaces/input_system.h"

#include <imgui.h>

// FIXME: Menu isn't rendering in-game
void Menu::render() {
  if (open && toggle_animation_end < 1.0f) {
    ImGui::SetNextWindowFocus();
  }

  ImGui::PushStyleVar(ImGuiStyleVar_Alpha, get_transparency());
  {
    toggle_animation_end += ImGui::GetIO().DeltaTime / animation_len();

    if (!open) {
      goto render_end;
    }
    ImGui::ShowDemoWindow();
  }
render_end:
  ImGui::PopStyleVar();
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
  }
}
