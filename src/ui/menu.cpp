#include "menu.h"

#include <interfaces/input_system.h>
#include <utils/input.h>

#include <app.h>

#include <imgui.h>

namespace ui {
  constexpr auto WINDOW_FLAGS = ImGuiWindowFlags_NoResize |
                                ImGuiWindowFlags_NoScrollbar |
                                ImGuiWindowFlags_AlwaysAutoResize;

  void Menu::draw() {
    if (open && toggle_animation_end < 1.0f) {
      ImGui::SetNextWindowFocus();
    } else if (!open) {
      return;
    }

    auto &cfg = App::get().config;

    ImGui::PushStyleVar(ImGuiStyleVar_Alpha, get_transparency());
    {
      draw_menu_bar();
      draw_misc_window(cfg);
    }
    ImGui::PopStyleVar();
  }

  void Menu::handle_toggle() {
    if (utils::Input::get().key_is_up(VK_INSERT)) {
      open = !open;
      App::with<void>([&](App &app) {
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

  void Menu::draw_menu_bar() {
    if (ImGui::BeginMainMenuBar()) {
      draw_menu_bar_item("Misc", should_draw_window.misc);
      ImGui::EndMainMenuBar();
    }
  }

  void Menu::draw_menu_bar_item(std::string_view window_name,
                                bool &should_draw_window) {
    const auto name = window_name.data();
    if (ImGui::MenuItem(name)) {
      should_draw_window = true;
      ImGui::SetWindowFocus(name);
    }
  }

  void Menu::draw_misc_window(Config &cfg) {
    if (!should_draw_window.misc) {
      return;
    }

    if (ImGui::Begin("Misc", &should_draw_window.misc, WINDOW_FLAGS)) {
      ImGui::Checkbox("Bunnyhop", &cfg.misc.bunnyhop.enabled);
      if (cfg.misc.bunnyhop.enabled) {
        ImGui::SameLine();
        ImGui::PushID("BunnyhopChance");
        ImGui::SliderFloat("Chance", &cfg.misc.bunnyhop.chance, 10.0f, 100.0f);
        ImGui::PopID();
      }

      ImGui::Checkbox("Aspect ratio", &cfg.misc.aspect_ratio.enabled);
      if (cfg.misc.aspect_ratio.enabled) {
        ImGui::SameLine();
        ImGui::PushID("AspectRatioValue");
        ImGui::SliderFloat("Value", &cfg.misc.aspect_ratio.value, 0.1f, 10.0f);
        ImGui::PopID();
      }
    }
    ImGui::End();
  }
}
