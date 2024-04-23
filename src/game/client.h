#pragma once

#include <utils/vmethod.h>

namespace game
{
  struct ClientClass;

  struct Client {
    VMETHOD(ClientClass *, get_all_classes, 8, (), (this))
  };
}
