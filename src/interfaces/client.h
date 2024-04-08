#pragma once

#include <utils/vmethod.h>

namespace game {
  struct ClientClass;
}

namespace interfaces {
  struct Client {
    VMETHOD(game::ClientClass *, get_all_classes, 8, (), (this))
  };
}
