#include "visuals.h"

#include <game/entity.h>

#include <app.h>

#include <imgui.h>

namespace hacks {
  void Visuals::draw_sniper_crosshair(ImDrawList *draw_list,
                                      const App &app) const {
    const auto &cfg = config.sniper_rifle_crosshair;
    if (!cfg.enabled || !app.local_player) {
      return;
    }

    if (const auto weapon = app.local_player->active_weapon();
        !weapon || !weapon->is_sniper_rifle() || weapon->is_in_scope()) {
      return;
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
