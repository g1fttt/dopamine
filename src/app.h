#pragma once

#include <Windows.h>

#include "ui/shared.h"
#include "utils/ptr.h"

#include <functional>
#include <memory>

namespace interfaces {
  struct Client;
  struct EntityList;
  struct Engine;
  struct CVar;
  struct InputSystem;
  struct Surface;
}

PRIVATE_USE(namespace interfaces)

namespace game {
  struct PlayerEntity;
}

struct Hooks;

struct App {
  struct Interfaces {
    Ptr<Client> client;
    Ptr<EntityList> entity_list;
    Ptr<Engine> engine;
    Ptr<CVar> cvar;
    Ptr<InputSystem> input_system;
    Ptr<Surface> surface;
    void *client_mode = nullptr;
  };

  ~App();

  // Use it only if program flow changes needed (e.g. return, goto)
  constexpr operator bool() const {
    return true;
  }

  static App &get_or_init(HINSTANCE inst_dll) {
    static App self{};
    if (inst_dll) {
      self.init_or_nothing(inst_dll);
    }
    return self;
  }

  static App &get() {
    return get_or_init(nullptr);
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

  Ptr<void> d3d9_present_raw;
  Ptr<void> d3d9_reset_raw;
private:
  App();

  void init_or_nothing(HMODULE module);
  void find_interfaces();
  void find_patterns();
};
