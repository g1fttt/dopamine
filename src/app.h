#pragma once

#include <Windows.h>

#include "hacks/glow/object_manager.h"
#include "hooks/hooks.h"
#include "ui/shared.h"

#include "entity_listener.h"
#include "interfaces.h"
#include "material_creator.h"
#include "netvars.h"
#include "patterns.h"

#include <memory>
#include <optional>

namespace game
{
  struct PlayerEntity;
}

namespace core
{
  struct App {
    App(HMODULE module);
    ~App();

    // MUST be called from any WinAPI function (Present, Reset, WndProc, etc.)
    void unload();

    bool should_anti_screenshot() const;

    HMODULE module = nullptr;
    HWND window = nullptr;

    std::optional<Hooks> hooks;
    std::optional<Interfaces> interfaces;

    std::optional<Netvars> netvars;
    std::optional<Patterns> patterns;

    std::optional<glow::ObjectManager> glow_object_manager;

    EntityListener entity_listener;
    MaterialCreator material_creator;

    // Obtained in `hooks::level_init_post_entity`
    game::PlayerEntity *local_player = nullptr;

    ui::ImGuiContext fore_imgui_ctx, back_imgui_ctx;
  };
}

constinit inline std::unique_ptr<core::App> app{};
