#pragma once

#include <utils/vmethod.h>

namespace interfaces {
  class InputSystem {
  public:
    VMETHOD(void, enable_input, 7, (bool enable), (this, enable))
    VMETHOD(void, reset_input_state, 25, (), (this))
  };
}
