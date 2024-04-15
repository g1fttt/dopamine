#include "menu.h"

#include <game/input_system.h>
#include <utils/input.h>

#include <hacks/glow/hack.h>

#include <hacks/misc.h>
#include <hacks/visuals.h>

#include <app.h>
#include <config.h>

#include <imgui.h>

template <typename T>
static void draw_feature(T &feat, const char *name, const char *id,
                         const std::function<void(T &)> &cb)
  requires requires {
    feat.enabled;
    config::Serde<T>;
  }
{
  ImGui::Checkbox(name, &feat.enabled);
  if (feat.enabled) {
    ImGui::SameLine();
    ImGui::PushID(id);
    { cb(feat); }
    ImGui::PopID();
  }
}

namespace ui {
  constexpr auto WINDOW_FLAGS = ImGuiWindowFlags_NoResize |
                                ImGuiWindowFlags_NoScrollbar |
                                ImGuiWindowFlags_AlwaysAutoResize;
  constexpr auto COLOR_EDIT_FLAGS =
      ImGuiColorEditFlags_AlphaBar | ImGuiColorEditFlags_NoInputs;

  void Menu::draw() {
    if (open && toggle_animation_end < 1.0f) {
      ImGui::SetNextWindowFocus();
    } else if (!open) {
      return;
    }

    ImGui::PushStyleVar(ImGuiStyleVar_Alpha, get_transparency());
    {
      draw_menu_bar();
      draw_misc_window();
      draw_visuals_window();
      draw_glow_window();
    }
    ImGui::PopStyleVar();
  }

  void Menu::handle_toggle(const utils::Input &input) {
    if (!input.key_is_up(VK_INSERT)) {
      return;
    }

    open = !open;
    App::get().and_then<void>([this](App &app) {
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

  void Menu::update_animation() {
    toggle_animation_end += ImGui::GetIO().DeltaTime / animation_len();
  }

  void Menu::draw_menu_bar() {
    if (ImGui::BeginMainMenuBar()) {
      draw_menu_bar_item("Misc", should_draw_window.misc);
      draw_menu_bar_item("Visuals", should_draw_window.visuals);
      draw_menu_bar_item("Glow", should_draw_window.glow);
      ImGui::EndMainMenuBar();
    }
  }

  void Menu::draw_menu_bar_item(const char *window_name,
                                bool &should_draw_window) {
    if (ImGui::MenuItem(window_name)) {
      should_draw_window = true;
      ImGui::SetWindowFocus(window_name);
    }
  }

  void Menu::draw_misc_window() {
    if (!should_draw_window.misc) {
      return;
    }

    auto &cfg = hacks::Misc::get().config;

    if (ImGui::Begin("Misc", &should_draw_window.misc, WINDOW_FLAGS)) {
      draw_feature<hacks::Misc::Bunnyhop>(
          cfg.bunnyhop, "Bunnyhop", "bunnyhop", [](auto &feat) {
            ImGui::SliderFloat("Chance", &feat.chance, 10.0f, 100.0f);
          });
    }
    ImGui::End();
  }

  void Menu::draw_visuals_window() {
    if (!should_draw_window.visuals) {
      return;
    }

    auto &cfg = hacks::Visuals::get().config;

    if (ImGui::Begin("Visuals", &should_draw_window.visuals, WINDOW_FLAGS)) {
      draw_feature<config::Feature<float>>(
          cfg.aspect_ratio, "Aspect ratio", "aspect_ratio", [](auto &feat) {
            ImGui::SliderFloat("Value", &feat.value, 0.5f, 5.0f);
          });

      draw_feature<config::Feature<float>>(
          cfg.add_fov, "Add FOV", "add_fov", [](auto &feat) {
            ImGui::SliderFloat("Value", &feat.value, -50.0f, 50.0f);
          });

      ImGui::Checkbox("Anti-screenshot", &cfg.anti_screenshot);

      draw_feature<hacks::Visuals::SniperRifleCrosshair>(
          cfg.sniper_rifle_crosshair, "Sniper rifle crosshair",
          "sniper_rifle_crosshair", [](auto &feat) {
            ImGui::SameLine();
            ImGui::ColorEdit4("##Color", feat.color.float_array(),
                              COLOR_EDIT_FLAGS);
            ImGui::SliderFloat("Size", &feat.size, 1.0f, 30.0f);
            ImGui::SliderFloat("Thickness", &feat.thickness, 1.0f, 5.0f);
          });
    }
    ImGui::End();
  }

  void Menu::draw_glow_window() {
    if (!should_draw_window.glow) {
      return;
    }

    auto &cfg = glow::Hack::get().config;

    if (ImGui::Begin("Glow", &should_draw_window.glow, WINDOW_FLAGS)) {
      draw_feature<glow::Hack::Glow>(
          cfg.enemies, "Enemies", "enemies", [](auto &feat) {
            ImGui::SameLine();
            ImGui::ColorEdit4("##Color", feat.color.float_array(),
                              COLOR_EDIT_FLAGS);
          });

      draw_feature<glow::Hack::Glow>(
          cfg.allies, "Allies", "allies", [](auto &feat) {
            ImGui::SameLine();
            ImGui::ColorEdit4("##Color", feat.color.float_array(),
                              COLOR_EDIT_FLAGS);
          });
    }
    ImGui::End();
  }
}
