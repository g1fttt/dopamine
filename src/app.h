#pragma once

#include <Windows.h>

#include "ui/shared.h"

#include "utils/ptr.h"

#include <memory>
#include <optional>

namespace game {
  struct PlayerEntity;
  struct Client;
  struct EntityList;
  struct Engine;
  struct CVar;
  struct InputSystem;
  struct Surface;
  struct RenderView;
  struct MaterialSystem;
  struct ModelRender;
}

struct Hooks;

struct App {
  struct Interfaces {
    utils::Ptr<game::Client> client;
    game::EntityList *entity_list = nullptr;
    game::Engine *engine = nullptr;
    game::CVar *cvar = nullptr;
    game::InputSystem *input_system = nullptr;
    game::Surface *surface = nullptr;
    game::RenderView *render_view;
    game::MaterialSystem *material_system;
    game::ModelRender *model_render;
    void *client_mode = nullptr;
  };

  App(HMODULE module);
  ~App();

  void reset();

  bool should_anti_screenshot() const;
  bool should_draw_visuals() const;

  HMODULE module = nullptr;
  HWND window = nullptr;

  // true if VK_END is pressed
  bool should_unload = false;

  // true if `should_unload` && `IDirect3DDevice9::Present` finished resetting
  bool must_unload = false;

  // Obtained in `hooks::level_init_post_entity`
  game::PlayerEntity *local_player = nullptr;

  // FIXME: Make them separate global variables
  Interfaces interfaces;
  std::unique_ptr<Hooks> hooks;

  ui::ImGuiContext fore_imgui_ctx, back_imgui_ctx;
private:
  void find_interfaces();
};

constinit inline std::optional<App> app{};
