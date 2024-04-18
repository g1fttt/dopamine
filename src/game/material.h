#pragma once

#include <utils/vmethod.h>

namespace game {
  struct MaterialVar;

  struct Material {
    VMETHOD(MaterialVar *, find_var, 11,
            (const char *var_name, bool *found = nullptr),
            (this, var_name, found, true))
    VMETHOD(void, inc_ref_counter, 12, (), (this))
  };
}
