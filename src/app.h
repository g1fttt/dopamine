#pragma once

#include <Windows.h>

#include <functional>

#include "menu.h"
#include "vmt.h"

struct ImGuiContext;

struct IDirect3DDevice9;

namespace interfaces {
  class CVar;
  class InputSystem;
  class Surface;
}

struct App {
  static App &get();

  static void with(const std::function<void(App &)> &cb);

  void reset();

  bool should_unhook = false;
  Menu menu;

  ImGuiContext *blur_ctx, *menu_ctx;

  WNDPROC original_wnd_proc;
  HWND window;

  VMT client_vmt;
  VMT d3d9_vmt;
  VMT surface_vmt;

  void *client;
  IDirect3DDevice9 *d3d9;
  interfaces::CVar *cvar;
  interfaces::InputSystem *input_system;
  interfaces::Surface *surface;
};
