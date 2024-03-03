#include "menu.h"

#include <Windows.h>

#include "app.h"

#include "interfaces/input_system.h"
#include "utils/input.h"

#include <imgui.h>

namespace core {
  Menu &Menu::get() {
    static Menu menu{};
    return menu;
  }

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
    if (utils::Input::get().key_is_up(VK_INSERT)) {
      App::with([&](App &app) {
        open = !open;
        if (!open) {
          app.interfaces.input_system->reset_input_state();
        }
        app.interfaces.input_system->enable_input(!open);
      });

      if (toggle_animation_end > 0.0f && toggle_animation_end < 1.0f) {
        toggle_animation_end = 1.0f - toggle_animation_end;
      } else {
        toggle_animation_end = 0.0f;
      }

      ImGui::GetIO().MouseDrawCursor = open;
      ShowCursor(!open);
    }
  }
}
