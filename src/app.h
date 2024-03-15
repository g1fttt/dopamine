#pragma once

#include <Windows.h>

#include <functional>
#include <type_traits>

#include "vmt.h"

struct IDirect3DDevice9;
struct _D3DPRESENT_PARAMETERS;

namespace interfaces {
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
    interfaces::EntityList *entity_list;
    interfaces::Engine *engine;
    interfaces::CVar *cvar;
    interfaces::InputSystem *input_system;
    interfaces::Surface *surface;
  };

  struct VMTs {
    core::VMT client_mode;
    core::VMT surface;
  };
public:
  static App &get();

  static void with(const std::function<void(App &)> &cb);

  void reset();
public:
  WNDPROC original_wnd_proc;
  HWND window;

  using D3D9_Present = HRESULT WINAPI(IDirect3DDevice9 *, const RECT *,
                                      const RECT *, HWND, const RGNDATA *);
  using D3D9_Reset = HRESULT WINAPI(IDirect3DDevice9 *,
                                    _D3DPRESENT_PARAMETERS *);

  std::add_pointer_t<D3D9_Present> d3d9_present_original;
  std::add_pointer_t<D3D9_Reset> d3d9_reset_original;

  // true if VK_END is pressed
  bool should_unhook = false;

  // true if `should_unhook` && IDirect3DDevice9::Present finished resetting
  bool must_unhook = false;

  internal::Entity *local_player;

  Interfaces interfaces;
  VMTs vmts;
private:
  void find_interfaces();
  void find_patterns();
  void init_vmts();
  void setup_hooks();
private:
  void *client_mode;

  void *d3d9_present_raw;
  void *d3d9_reset_raw;
};
