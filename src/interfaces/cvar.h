#pragma once

#include <utils/vmethod.h>

namespace internal {
  class ConVar;
}

namespace interfaces {
  class CVar {
  public:
    VMETHOD(internal::ConVar *, find_var, 12, (const char *name), (this, name))
  };
}
