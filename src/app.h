#pragma once

#include <Windows.h>

#include <functional>

#include "vmt.h"

struct ImGuiContext;

struct IDirect3DDevice9;

namespace interfaces {
  class CVar;
  class InputSystem;
  class Surface;
}

class App {
public:
  struct Interfaces {
    void *client;
    IDirect3DDevice9 *d3d9;
    interfaces::CVar *cvar;
    interfaces::InputSystem *input_system;
    interfaces::Surface *surface;
  };

  struct VMTs {
    VMT client;
    VMT d3d9;
    VMT surface;
  };
public:
  static App &get();

  static void with(const std::function<void(App &)> &cb);

  void reset();

  bool should_unhook = false;

  Interfaces interfaces;
  VMTs vmts;

  ImGuiContext *blur_ctx, *menu_ctx;

  WNDPROC original_wnd_proc;
  HWND window;
};
