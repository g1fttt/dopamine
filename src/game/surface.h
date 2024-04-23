#pragma once

#include <utils/vmethod.h>

namespace game
{
  struct Surface {
    VMETHOD(bool, is_cursor_visible, 53, (), (this))
    VMETHOD(void, unlock_cursor, 61, (), (this))
  };
}
