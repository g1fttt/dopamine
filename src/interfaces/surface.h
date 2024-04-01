#pragma once

#include <utils/vmethod.h>

namespace interfaces {
  struct Surface {
    VMETHOD(void, unlock_cursor, 61, (), (this))
  };
}
