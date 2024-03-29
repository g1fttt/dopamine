#pragma once

#include <Windows.h>

#include "utils/ptr.h"
#include "utils/vmt.h"

#include "config.h"

struct IDirect3DDevice9;
struct _D3DPRESENT_PARAMETERS;

namespace interfaces {
  class Client;
  class EntityList;
  class Engine;
  class CVar;
  class InputSystem;
  class Surface;
}

namespace internal {
  class Entity;
}

class App {
public:
  struct Interfaces {
    Ptr<interfaces::Client> client;
    Ptr<interfaces::EntityList> entity_list;
    Ptr<interfaces::Engine> engine;
    Ptr<interfaces::CVar> cvar;
    Ptr<interfaces::InputSystem> input_system;
    Ptr<interfaces::Surface> surface;
  };

  struct VMTs {
    utils::VMT client_mode;
    utils::VMT surface;
    utils::VMT engine;
  };

  using D3D9_Present = HRESULT WINAPI(IDirect3DDevice9 *, const RECT *,
                                      const RECT *, HWND, const RGNDATA *);
  using D3D9_Reset = HRESULT WINAPI(IDirect3DDevice9 *,
                                    _D3DPRESENT_PARAMETERS *);
public:
  constexpr App(const App &&) = delete;
  constexpr App(const App &) = delete;

  static App &get() {
    static App self{};
    { self.init_or_nothing(); }
    return self;
  }

  template <typename T> static T with(const std::function<T(App &)> &cb) {
    return cb(App::get());
  }

  void reset();
public:
  WNDPROC original_wnd_proc;
  HWND window;

  std::add_pointer_t<D3D9_Present> d3d9_present_original;
  std::add_pointer_t<D3D9_Reset> d3d9_reset_original;

  // true if VK_END is pressed
  bool should_unhook = false;

  // true if `should_unhook` && IDirect3DDevice9::Present finished resetting
  bool must_unhook = false;

  internal::Entity *local_player;

  Config config;

  Interfaces interfaces;
  VMTs vmts;
private:
  constexpr App() = default;

  void init_or_nothing();
  void find_interfaces();
  void find_patterns();
  void init_vmts();
  void setup_hooks();
private:
  void *client_mode;

  Ptr<void> d3d9_present_raw;
  Ptr<void> d3d9_reset_raw;
};
