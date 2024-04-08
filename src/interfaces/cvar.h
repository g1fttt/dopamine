#pragma once

#include <utils/vmethod.h>

namespace game {
  struct ConVar;
}

namespace interfaces {
  struct CVar {
    VMETHOD(game::ConVar *, find_var, 12, (const char *name), (this, name))
  };
}
