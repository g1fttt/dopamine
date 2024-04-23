#include "visuals.h"

#include <game/entity.h>
#include <game/view.h>

#include <app.h>

#include <imgui.h>

namespace hacks
{
  void Visuals::draw_sniper_crosshair(game::PlayerEntity *local_player,
                                      ImDrawList *draw_list) const {
    const auto &cfg = config.sniper_rifle_crosshair;
    if (!cfg.enabled || !local_player) {
      return;
    }

    if (const auto weapon = local_player->active_weapon();
        !weapon || !weapon->is_sniper_rifle() || weapon->is_in_scope())
    {
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

  void Visuals::override_fov(game::ViewSetup *view) const {
    const auto &cfg = config.add_fov;
    if (!cfg.enabled || app->should_anti_screenshot()) {
      return;
    }

    view->fov += cfg.value;
  }
}
