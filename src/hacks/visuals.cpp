#include "visuals.h"

#include <internal/entity.h>

#include <app.h>

#include <imgui.h>

namespace hacks {
  void Visuals::draw_sniper_crosshair(ImDrawList *draw_list) const {
    const auto &cfg = config.sniper_rifle_crosshair;

    if (!cfg.enabled) {
      return;
    }

    if (const App &app = App::get()) {
      if (!app.local_player) {
        return;
      }

      if (auto *weapon = app.local_player->active_weapon();
          !weapon || !weapon->is_sniper_rifle()) {
        return;
      }
    }

    const auto [display_width, display_height] = ImGui::GetIO().DisplaySize;
    const ImVec2 display_center = {display_width / 2.0f, display_height / 2.0f};

    // Vertical
    draw_list->AddRectFilled(
        {display_center.x - cfg.thickness, display_center.y - cfg.size},
        {display_center.x + cfg.thickness, display_center.y + cfg.size},
        cfg.color.im_u32());

    // Horizontal
    draw_list->AddRectFilled(
        {display_center.x - cfg.size, display_center.y - cfg.thickness},
        {display_center.x + cfg.size, display_center.y + cfg.thickness},
        cfg.color.im_u32());
  }
}
