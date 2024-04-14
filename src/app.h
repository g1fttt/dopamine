#pragma once

#include <Windows.h>

#include "ui/shared.h"

#include "utils/ptr.h"
#include "utils/singleton.h"

#include <memory>

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

struct App : utils::Singleton<App> {
  friend struct Singleton<App>;

  struct Interfaces {
    utils::Ptr<game::Client> client;
    utils::Ptr<game::EntityList> entity_list;
    utils::Ptr<game::Engine> engine;
    utils::Ptr<game::CVar> cvar;
    utils::Ptr<game::InputSystem> input_system;
    utils::Ptr<game::Surface> surface;
    utils::Ptr<game::RenderView> render_view;
    utils::Ptr<game::MaterialSystem> material_system;
    utils::Ptr<game::ModelRender> model_render;
    void *client_mode = nullptr;
  };

  ~App();

  // Use it only if program flow changes needed (e.g. return, goto)
  constexpr operator bool() const {
    return true;
  }

  static Singleton<App>::InitFunc init_func(HMODULE module) {
    return [=](App &app) {
      app.init_or_nothing(module);
    };
  }

  template <typename T>
  constexpr T and_then(const std::function<T(App &)> &cb) {
    return cb(*this);
  }

  void reset();

  bool should_anti_screenshot() const;
  bool should_draw_visuals() const;

  HMODULE module = nullptr;
  HWND window = nullptr;

  // true if VK_END is pressed
  bool should_unhook = false;

  // true if `should_unhook` && `IDirect3DDevice9::Present` finished resetting
  bool must_unhook = false;

  // Obtained in `hooks::level_init_post_entity`
  game::PlayerEntity *local_player = nullptr;

  Interfaces interfaces;
  std::unique_ptr<Hooks> hooks;

  ui::ImGuiContext fore_imgui_ctx, back_imgui_ctx;

  utils::Ptr<void> d3d9_present_raw;
  utils::Ptr<void> d3d9_reset_raw;
private:
  App();

  void init_or_nothing(HMODULE module);
  void find_interfaces();
  void find_patterns();
};
