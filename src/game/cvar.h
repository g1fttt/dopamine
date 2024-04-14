#pragma once

#include <utils/vmethod.h>

namespace game {
  struct ConVar;

  struct CVar {
    VMETHOD(ConVar *, find_var, 12, (const char *name), (this, name))
  };
}
