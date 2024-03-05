#pragma once

#include <Windows.h>

#include <functional>

#include "vmt.h"

struct IDirect3DDevice9;

namespace interfaces {
  class CVar;
  class InputSystem;
  class Surface;
}

class App {
public:
  struct Interfaces {
    IDirect3DDevice9 *d3d9;
    interfaces::CVar *cvar;
    interfaces::InputSystem *input_system;
    interfaces::Surface *surface;
  };

  struct VMTs {
    core::VMT d3d9;
    core::VMT surface;
  };
public:
  static App &get();

  static void with(const std::function<void(App &)> &cb);

  void reset();

  // true if VK_END is pressed
  bool should_unhook = false;

  // true if `should_unhook` && IDirect3DDevice9::Present finished resetting
  bool must_unhook = false;

  Interfaces interfaces;
  VMTs vmts;

  WNDPROC original_wnd_proc;
  HWND window;
};
