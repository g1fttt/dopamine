#pragma once

#include "vmethod.h"

class ConVar;

class CVar {
public:
  VMETHOD(ConVar *, find_var, 12, (const char *name), (this, name))
};
