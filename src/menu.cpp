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

    constexpr auto WINDOW_FLAGS = ImGuiWindowFlags_NoCollapse |
                                  ImGuiWindowFlags_NoResize |
                                  ImGuiWindowFlags_NoScrollbar;

    ImGui::Begin("Dopamine", nullptr, WINDOW_FLAGS);
    ImGui::Text("Hello, World!");
    ImGui::End();
  }
end:
  ImGui::PopStyleVar();
}

void Menu::update_animation() {
  toggle_animation_end += ImGui::GetIO().DeltaTime / animation_len();
}

void Menu::handle_toggle() {
  App::with([&](App &app) {
    if (app.input.key_is_up(VK_INSERT)) {
      open = !open;
      if (!open) {
        app.input_system->reset_input_state();
      }

      if (toggle_animation_end > 0.0f && toggle_animation_end < 1.0f) {
        toggle_animation_end = 1.0f - toggle_animation_end;
      } else {
        toggle_animation_end = 0.0f;
      }

      ImGui::GetIO().MouseDrawCursor = open;
      ShowCursor(!open);
    }
  });
}
