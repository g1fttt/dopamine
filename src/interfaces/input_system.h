#pragma once

#include <utils/vmethod.h>

namespace interfaces {
  struct InputSystem {
    VMETHOD(void, enable_input, 7, (bool enable), (this, enable))
    VMETHOD(void, reset_input_state, 25, (), (this))
  };
}
