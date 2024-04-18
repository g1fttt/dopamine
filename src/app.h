#pragma once

#include <Windows.h>

#include "ui/shared.h"

#include <optional>

namespace game {
  struct PlayerEntity;
}

namespace core {
  struct App {
    App(HMODULE module);

    void reset();

    bool should_anti_screenshot() const;

    HMODULE module = nullptr;
    HWND window = nullptr;

    // true if VK_END is pressed
    bool should_unload = false;

    // true if `should_unload` && `IDirect3DDevice9::Present` finished resetting
    bool must_unload = false;

    // Obtained in `hooks::level_init_post_entity`
    game::PlayerEntity *local_player = nullptr;

    ui::ImGuiContext fore_imgui_ctx, back_imgui_ctx;
  };

  constinit inline std::optional<App> app{};
}
