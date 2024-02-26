#pragma once

#include <functional>

#include "vmt.h"

namespace interfaces {
  class CVar;
}

struct App {
  static App &get();
  static void with(const std::function<void(App &)> &cb);

  bool should_unhook = false;

  VMT client_vmt;

  void *client;
  interfaces::CVar *cvar;
};
