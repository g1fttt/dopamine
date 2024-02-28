#pragma once

#include <Windows.h>

#include <functional>

#include "vmt.h"

struct IDirect3DDevice9;

namespace interfaces {
  class CVar;
}

struct App {
  static App &get();
  static void with(const std::function<void(App &)> &cb);

  void reset();

  bool should_unhook = false;

  WNDPROC original_wnd_proc;
  HWND window;

  VMT client_vmt, d3d9_vmt;

  void *client;
  IDirect3DDevice9 *d3d9;
  interfaces::CVar *cvar;
};
