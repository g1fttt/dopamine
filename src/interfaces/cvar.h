#pragma once

#include <utils/vmethod.h>

namespace internal {
  struct ConVar;
}

namespace interfaces {
  struct CVar {
    VMETHOD(internal::ConVar *, find_var, 12, (const char *name), (this, name))
  };
}
