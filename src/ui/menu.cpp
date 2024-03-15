#include "menu.h"

#include <Windows.h>

#include <interfaces/input_system.h>
#include <utils/input.h>

#include <app.h>

#include <imgui.h>

namespace ui {
  Menu &Menu::get() {
    static Menu self{};
    return self;
  }

  void Menu::draw() const {
    if (open && toggle_animation_end < 1.0f) {
      ImGui::SetNextWindowFocus();
    }

    ImGui::PushStyleVar(ImGuiStyleVar_Alpha, get_transparency());
    {
      if (!open) {
        goto end;
      }

      constexpr auto WINDOW_FLAGS =
          ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoResize |
          ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_AlwaysAutoResize;

      ImGui::Begin("Dopamine", nullptr, WINDOW_FLAGS);
      ImGui::Text("Hello, World!");
      ImGui::End();
    }
  end:
    ImGui::PopStyleVar();
  }

  void Menu::handle_toggle() {
    if (utils::Input::get().key_is_up(VK_INSERT)) {
      open = !open;
      App::with([&](App &app) {
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

  void Menu::update_animation() {
    toggle_animation_end += ImGui::GetIO().DeltaTime / animation_len();
  }
}
