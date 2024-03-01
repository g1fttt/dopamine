#pragma once

#include <utils/vmethod.h>

namespace interfaces {
  class Surface {
  public:
    VMETHOD(void, unlock_cursor, 61, (), (this))
  };
}
