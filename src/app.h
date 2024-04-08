#pragma once

#include <Windows.h>

#include "utils/ptr.h"
#include "utils/vmt.h"

#include "ui/shared.h"

#include <functional>

struct IDirect3DDevice9;
struct _D3DPRESENT_PARAMETERS;

namespace interfaces {
  struct Client;
  struct EntityList;
  struct Engine;
  struct CVar;
  struct InputSystem;
  struct Surface;
}

namespace internal {
  struct PlayerEntity;
  struct UserCommand;
}

namespace {
  using namespace interfaces;
  using namespace utils;
}

struct App {
  struct Interfaces {
    Ptr<Client> client;
    Ptr<EntityList> entity_list;
    Ptr<Engine> engine;
    Ptr<CVar> cvar;
    Ptr<InputSystem> input_system;
    Ptr<Surface> surface;
  };

  struct VMTs {
    VMT client_mode;
    VMT surface;
    VMT engine;
    VMT client;
  };

  using D3D9_Present = HRESULT WINAPI(IDirect3DDevice9 *, const RECT *,
                                      const RECT *, HWND, const RGNDATA *);
  using D3D9_Reset = HRESULT WINAPI(IDirect3DDevice9 *,
                                    _D3DPRESENT_PARAMETERS *);

  constexpr App(const App &&) = delete;
  constexpr App(const App &) = delete;

  // Use it only if program flow changes needed (e.g. return, goto)
  constexpr operator bool() const {
    return true;
  }

  static App &get() {
    static App self{};
    { self.init_or_nothing(); }
    return self;
  }

  template <typename T>
  constexpr static T with(const std::function<T(App &)> &cb) {
    return cb(App::get());
  }

  void reset();

  bool should_anti_screenshot() const;
  bool should_draw_visuals() const;

  WNDPROC original_wnd_proc = nullptr;
  HWND window = nullptr;

  std::add_pointer_t<D3D9_Present> d3d9_present_original = nullptr;
  std::add_pointer_t<D3D9_Reset> d3d9_reset_original = nullptr;

  // true if VK_END is pressed
  bool should_unhook = false;

  // true if `should_unhook` && `IDirect3DDevice9::Present` finished resetting
  bool must_unhook = false;

  // Received from `hooks::frame_stage_notify`
  internal::PlayerEntity *local_player = nullptr;

  Interfaces interfaces;
  VMTs vmts;

  ui::ImGuiContext fore_imgui_ctx, back_imgui_ctx;
private:
  constexpr App() = default;

  void init_or_nothing();
  void find_interfaces();
  void find_patterns();
  void init_vmts();
  void setup_hooks();

  void *client_mode = nullptr;

  Ptr<void> d3d9_present_raw;
  Ptr<void> d3d9_reset_raw;
};
